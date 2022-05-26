// this whole file was taken from https://github.com/mvdnes/rboy/blob/master/src/sound.rs

use blip_buf::BlipBuf;
use maikor_platform::mem;

const WAVE_PATTERN: [[i32; 8]; 4] = [
    [-1, -1, -1, -1, 1, -1, -1, -1],
    [-1, -1, -1, -1, 1, 1, -1, -1],
    [-1, -1, 1, 1, 1, 1, -1, -1],
    [1, 1, 1, 1, -1, -1, 1, 1],
];
const CLOCKS_PER_SECOND: u32 = 1 << 22;
const OUTPUT_SAMPLE_COUNT: usize = 2000; // this should be less than blip_buf::MAX_FRAME

const ADDR: u16 = mem::address::SOUND;
const S1_S: u16 = ADDR;
const S1_D: u16 = ADDR + 1;
const S1_V: u16 = ADDR + 2;
const S1_F: u16 = ADDR + 3;
const S1_T: u16 = ADDR + 4;
//UNUSED 5
const S2_D: u16 = ADDR + 6;
const S2_V: u16 = ADDR + 7;
const S2_F: u16 = ADDR + 8;
const S2_T: u16 = ADDR + 9;
const W_P: u16 = ADDR + 10;
const W_L: u16 = ADDR + 11;
const W_V: u16 = ADDR + 12;
const W_F: u16 = ADDR + 13;
const W_T: u16 = ADDR + 14;
//UNUSED 15
const N_L: u16 = ADDR + 16;
const N_V: u16 = ADDR + 17;
const N_C: u16 = ADDR + 18;
const N_T: u16 = ADDR + 19;
const C_V: u16 = ADDR + 20;
const C_B: u16 = ADDR + 21;
const C_P: u16 = ADDR + 22;

const WAVE_TABLE_START: u16 = mem::address::WAVE_TABLE;
const WAVE_TABLE_END: u16 = mem::address::WAVE_TABLE + mem::sizes::WAVE_TABLE;

const SOUND: usize = mem::address::SOUND as usize;
const WAVE_TABLE: usize = mem::address::WAVE_TABLE as usize;

pub trait AudioPlayer: Send {
    fn play(&mut self, left_channel: &[f32], right_channel: &[f32]);
    fn samples_rate(&self) -> u32;
    fn underflowed(&self) -> bool;
}

struct VolumeEnvelope {
    period: u8,
    goes_up: bool,
    delay: u8,
    initial_volume: u8,
    volume: u8,
}

impl VolumeEnvelope {
    fn new() -> VolumeEnvelope {
        VolumeEnvelope {
            period: 0,
            goes_up: false,
            delay: 0,
            initial_volume: 0,
            volume: 0,
        }
    }

    fn update(&mut self, addr: u16, value: u8) {
        match addr {
            S1_V | S2_V | N_V => {
                self.period = value & 0x7;
                self.goes_up = value & 0x8 == 0x8;
                self.initial_volume = value >> 4;
                self.volume = self.initial_volume;
            }
            S1_T | S2_T | N_T if value & 0x80 == 0x80 => {
                self.delay = self.period;
                self.volume = self.initial_volume;
            }
            _ => (),
        }
    }

    fn step(&mut self) {
        match self.delay {
            0 => {}
            1 => {
                self.delay = self.period;
                if self.goes_up && self.volume < 15 {
                    self.volume += 1;
                } else if !self.goes_up && self.volume > 0 {
                    self.volume -= 1;
                }
            }
            _ => self.delay -= 1,
        }
    }
}

struct SquareChannel {
    enabled: bool,
    duty: u8,
    phase: u8,
    length: u8,
    new_length: u8,
    length_enabled: bool,
    frequency: u16,
    period: u32,
    last_amp: i32,
    delay: u32,
    has_sweep: bool,
    sweep_frequency: u16,
    sweep_delay: u8,
    sweep_period: u8,
    sweep_shift: u8,
    sweep_frequency_increase: bool,
    volume_envelope: VolumeEnvelope,
    blip: BlipBuf,
}

impl SquareChannel {
    fn new(blip: BlipBuf, with_sweep: bool) -> SquareChannel {
        SquareChannel {
            enabled: false,
            duty: 1,
            phase: 1,
            length: 0,
            new_length: 0,
            length_enabled: false,
            frequency: 0,
            period: 2048,
            last_amp: 0,
            delay: 0,
            has_sweep: with_sweep,
            sweep_frequency: 0,
            sweep_delay: 0,
            sweep_period: 0,
            sweep_shift: 0,
            sweep_frequency_increase: false,
            volume_envelope: VolumeEnvelope::new(),
            blip,
        }
    }

    fn is_on(&self) -> bool {
        self.enabled
    }

    fn update(&mut self, addr: u16, value: u8) {
        match addr {
            S1_S => {
                self.sweep_period = (value >> 4) & 0x7;
                self.sweep_shift = value & 0x7;
                self.sweep_frequency_increase = value & 0x8 == 0x8;
            }
            S1_D | S2_D => {
                self.duty = value >> 6;
                self.new_length = 64 - (value & 0x3F);
            }
            S1_F | S2_F => {
                self.frequency = (self.frequency & 0x0700) | (value as u16);
                self.length = self.new_length;
                self.calculate_period();
            }
            S1_T | S2_T => {
                self.frequency = (self.frequency & 0x00FF) | (((value & 0b0000_0111) as u16) << 8);
                self.calculate_period();
                self.length_enabled = value & 0x40 == 0x40;

                if value & 0x80 == 0x80 {
                    self.enabled = true;
                    self.length = self.new_length;

                    self.sweep_frequency = self.frequency;
                    if self.has_sweep && self.sweep_period > 0 && self.sweep_shift > 0 {
                        self.sweep_delay = 1;
                        self.step_sweep();
                    }
                }
            }
            _ => (),
        }
        self.volume_envelope.update(addr, value);
    }

    fn calculate_period(&mut self) {
        if self.frequency > 2048 {
            self.period = 0;
        } else {
            self.period = (2048 - self.frequency as u32) * 4;
        }
    }

    // This assumes no volume or sweep adjustments need to be done in the meantime
    fn run(&mut self, start_time: u32, end_time: u32) {
        if !self.enabled || self.period == 0 || self.volume_envelope.volume == 0 {
            if self.last_amp != 0 {
                self.blip.add_delta(start_time, -self.last_amp);
                self.last_amp = 0;
                self.delay = 0;
            }
        } else {
            let mut time = start_time + self.delay;
            let pattern = WAVE_PATTERN[self.duty as usize];
            let vol = self.volume_envelope.volume as i32;

            while time < end_time {
                let amp = vol * pattern[self.phase as usize];
                if amp != self.last_amp {
                    self.blip.add_delta(time, amp - self.last_amp);
                    self.last_amp = amp;
                }
                time += self.period;
                self.phase = (self.phase + 1) % 8;
            }

            // next time, we have to wait an additional delay timesteps
            self.delay = time - end_time;
        }
    }

    fn step_length(&mut self) {
        if self.length_enabled && self.length != 0 {
            self.length -= 1;
            if self.length == 0 {
                self.enabled = false;
            }
        }
    }

    fn step_sweep(&mut self) {
        if !self.has_sweep || self.sweep_period == 0 {
            return;
        }

        if self.sweep_delay > 1 {
            self.sweep_delay -= 1;
        } else {
            self.sweep_delay = self.sweep_period;
            self.frequency = self.sweep_frequency;
            if self.frequency == 2048 {
                self.enabled = false;
            }
            self.calculate_period();

            let offset = self.sweep_frequency >> self.sweep_shift;

            if self.sweep_frequency_increase {
                // F ~ (2048 - f)
                // Increase in frequency means subtracting the offset
                if self.sweep_frequency <= offset {
                    self.sweep_frequency = 0;
                } else {
                    self.sweep_frequency -= offset;
                }
            } else if self.sweep_frequency >= 2048 - offset {
                self.sweep_frequency = 2048;
            } else {
                self.sweep_frequency += offset;
            }
        }
    }
}

struct WaveChannel {
    enabled: bool,
    enabled_flag: bool,
    length: u16,
    new_length: u16,
    length_enabled: bool,
    frequency: u16,
    period: u32,
    last_amp: i32,
    delay: u32,
    volume_shift: u8,
    waveram: [u8; 32],
    current_wave: u8,
    blip: BlipBuf,
}

impl WaveChannel {
    fn new(blip: BlipBuf) -> WaveChannel {
        WaveChannel {
            enabled: false,
            enabled_flag: false,
            length: 0,
            new_length: 0,
            length_enabled: false,
            frequency: 0,
            period: 2048,
            last_amp: 0,
            delay: 0,
            volume_shift: 0,
            waveram: [0; 32],
            current_wave: 0,
            blip,
        }
    }

    fn update(&mut self, addr: u16, value: u8) {
        match addr {
            W_P => {
                self.enabled_flag = (value & 0x80) == 0x80;
                self.enabled = self.enabled && self.enabled_flag;
            }
            W_L => self.new_length = 256 - (value as u16),
            W_V => self.volume_shift = (value >> 5) & 0b11,
            W_F => {
                self.frequency = (self.frequency & 0x0700) | (value as u16);
                self.calculate_period();
            }
            W_T => {
                self.frequency = (self.frequency & 0x00FF) | (((value & 0b111) as u16) << 8);
                self.calculate_period();
                self.length_enabled = value & 0x40 == 0x40;
                if value & 0x80 == 0x80 && self.enabled_flag {
                    self.length = self.new_length;
                    self.enabled = true;
                    self.current_wave = 0;
                    self.delay = 0;
                }
            }
            WAVE_TABLE_START..=WAVE_TABLE_END => {
                self.waveram[(addr as usize - WAVE_TABLE) * 2] = value >> 4;
                self.waveram[(addr as usize - WAVE_TABLE) * 2 + 1] = value & 0xF;
            }
            _ => (),
        }
    }

    fn calculate_period(&mut self) {
        if self.frequency > 2048 {
            self.period = 0;
        } else {
            self.period = (2048 - self.frequency as u32) * 2;
        }
    }

    fn is_on(&self) -> bool {
        self.enabled
    }

    fn run(&mut self, start_time: u32, end_time: u32) {
        if !self.enabled || self.period == 0 {
            if self.last_amp != 0 {
                self.blip.add_delta(start_time, -self.last_amp);
                self.last_amp = 0;
                self.delay = 0;
            }
        } else {
            let mut time = start_time + self.delay;

            // A sample may be muted, 100%, 50% or 25%.
            // To avoid loss of precision, the WaveChannel will output
            // i32 samples at 4x the usual amplitude. This will be taken
            // into account when mixing all the samples.
            let volshift = match self.volume_shift {
                0 => 4 + 2, // to mute a 4 bit sample mutiplied by 2^2
                1 => 0,
                2 => 1,
                3 => 2,
                _ => unreachable!(),
            };

            while time < end_time {
                let sample = self.waveram[self.current_wave as usize];

                // shifted by 2 so that 25% does not lose precision
                let amp = ((sample << 2) >> volshift) as i32;

                if amp != self.last_amp {
                    self.blip.add_delta(time, amp - self.last_amp);
                    self.last_amp = amp;
                }

                time += self.period;
                self.current_wave = (self.current_wave + 1) % 32;
            }

            // next time, we have to wait an additional delay timesteps
            self.delay = time - end_time;
        }
    }

    fn step_length(&mut self) {
        if self.length_enabled && self.length != 0 {
            self.length -= 1;
            if self.length == 0 {
                self.enabled = false;
            }
        }
    }
}

struct NoiseChannel {
    enabled: bool,
    length: u8,
    new_length: u8,
    length_enabled: bool,
    volume_envelope: VolumeEnvelope,
    period: u32,
    shift_width: u8,
    state: u16,
    delay: u32,
    last_amp: i32,
    blip: BlipBuf,
}

impl NoiseChannel {
    fn new(blip: BlipBuf) -> NoiseChannel {
        NoiseChannel {
            enabled: false,
            length: 0,
            new_length: 0,
            length_enabled: false,
            volume_envelope: VolumeEnvelope::new(),
            period: 2048,
            shift_width: 14,
            state: 1,
            delay: 0,
            last_amp: 0,
            blip,
        }
    }

    fn update(&mut self, addr: u16, value: u8) {
        match addr {
            N_L => self.new_length = 64 - (value & 0x3F),
            N_V => (),
            N_C => {
                self.shift_width = if value & 8 == 8 { 6 } else { 14 };
                let freq_div = match value & 7 {
                    0 => 8,
                    n => (n as u32 + 1) * 16,
                };
                self.period = freq_div << (value >> 4);
            }
            N_T => {
                self.length_enabled = value & 0x40 == 0x40;
                if value & 0x80 == 0x80 {
                    self.enabled = true;
                    self.length = self.new_length;
                    self.state = 0xFF;
                    self.delay = 0;
                }
            }
            _ => (),
        }
        self.volume_envelope.update(addr, value);
    }

    fn is_on(&self) -> bool {
        self.enabled
    }

    fn run(&mut self, start_time: u32, end_time: u32) {
        if !self.enabled || self.volume_envelope.volume == 0 {
            if self.last_amp != 0 {
                self.blip.add_delta(start_time, -self.last_amp);
                self.last_amp = 0;
                self.delay = 0;
            }
        } else {
            let mut time = start_time + self.delay;
            while time < end_time {
                let oldstate = self.state;
                self.state <<= 1;
                let bit = ((oldstate >> self.shift_width) ^ (self.state >> self.shift_width)) & 1;
                self.state |= bit;

                let amp = match (oldstate >> self.shift_width) & 1 {
                    0 => -(self.volume_envelope.volume as i32),
                    _ => self.volume_envelope.volume as i32,
                };

                if self.last_amp != amp {
                    self.blip.add_delta(time, amp - self.last_amp);
                    self.last_amp = amp;
                }

                time += self.period;
            }
            self.delay = time - end_time;
        }
    }

    fn step_length(&mut self) {
        if self.length_enabled && self.length != 0 {
            self.length -= 1;
            if self.length == 0 {
                self.enabled = false;
            }
        }
    }
}

pub struct Sound {
    //if sound is enabled
    on: bool,
    //duplicate of sound register memory
    register_data: [u8; 0x17],
    time: u32,
    prev_time: u32,
    next_time: u32,
    time_divider: u8,
    output_period: u32,
    channel1: SquareChannel,
    channel2: SquareChannel,
    channel3: WaveChannel,
    channel4: NoiseChannel,
    volume_left: u8,
    volume_right: u8,
    player: Box<dyn AudioPlayer>,
}

impl Sound {
    pub fn new(player: Box<dyn AudioPlayer>) -> Sound {
        let blipbuf1 = create_blipbuf(player.samples_rate());
        let blipbuf2 = create_blipbuf(player.samples_rate());
        let blipbuf3 = create_blipbuf(player.samples_rate());
        let blipbuf4 = create_blipbuf(player.samples_rate());

        let output_period =
            (OUTPUT_SAMPLE_COUNT as u64 * CLOCKS_PER_SECOND as u64) / player.samples_rate() as u64;

        Sound {
            on: false,
            register_data: [0; 0x17],
            time: 0,
            prev_time: 0,
            next_time: CLOCKS_PER_SECOND / 256,
            time_divider: 0,
            output_period: output_period as u32,
            channel1: SquareChannel::new(blipbuf1, true),
            channel2: SquareChannel::new(blipbuf2, false),
            channel3: WaveChannel::new(blipbuf3),
            channel4: NoiseChannel::new(blipbuf4),
            volume_left: 7,
            volume_right: 7,
            player,
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        self.run();
        match addr {
            ADDR..=C_B => self.register_data[addr as usize - SOUND],
            C_P => {
                (self.register_data[addr as usize - SOUND] & 0xF0)
                    | (if self.channel1.is_on() { 1 } else { 0 })
                    | (if self.channel2.is_on() { 2 } else { 0 })
                    | (if self.channel3.is_on() { 4 } else { 0 })
                    | (if self.channel4.is_on() { 8 } else { 0 })
            }
            WAVE_TABLE_START..=WAVE_TABLE_END => {
                (self.channel3.waveram[(addr as usize - WAVE_TABLE) * 2] << 4)
                    | self.channel3.waveram[(addr as usize - WAVE_TABLE) * 2 + 1]
            }
            _ => 0,
        }
    }

    pub fn update(&mut self, addr: u16, value: u8) {
        if addr != C_P && !self.on {
            return;
        }
        self.run();
        #[allow(clippy::manual_range_contains)] //range is 2x slower
        if addr >= S1_S && addr <= C_P {
            self.register_data[addr as usize - SOUND] = value;
        }
        match addr {
            S1_S..=S1_T => self.channel1.update(addr, value),
            S2_D..=S2_T => self.channel2.update(addr, value),
            W_P..=W_T => self.channel3.update(addr, value),
            N_L..=N_T => self.channel4.update(addr, value),
            C_V => {
                self.volume_left = value & 0x7;
                self.volume_right = (value >> 4) & 0x7;
            }
            C_P => self.on = value & 0x80 == 0x80,
            WAVE_TABLE_START..=WAVE_TABLE_END => self.channel3.update(addr, value),
            _ => (),
        }
    }

    pub fn do_cycle(&mut self, cycles: u32) {
        if !self.on {
            return;
        }

        self.time += cycles;

        if self.time >= self.output_period {
            self.do_output();
        }
    }

    fn do_output(&mut self) {
        self.run();
        debug_assert!(self.time == self.prev_time);
        self.channel1.blip.end_frame(self.time);
        self.channel2.blip.end_frame(self.time);
        self.channel3.blip.end_frame(self.time);
        self.channel4.blip.end_frame(self.time);
        self.next_time -= self.time;
        self.time = 0;
        self.prev_time = 0;

        if self.player.underflowed() {
            self.mix_buffers();
        } else {
            // Prevent the BlipBuf's from filling up and triggering an assertion
            self.clear_buffers();
        }
    }

    fn run(&mut self) {
        while self.next_time <= self.time {
            self.channel1.run(self.prev_time, self.next_time);
            self.channel2.run(self.prev_time, self.next_time);
            self.channel3.run(self.prev_time, self.next_time);
            self.channel4.run(self.prev_time, self.next_time);

            self.channel1.step_length();
            self.channel2.step_length();
            self.channel3.step_length();
            self.channel4.step_length();

            if self.time_divider == 0 {
                self.channel1.volume_envelope.step();
                self.channel2.volume_envelope.step();
                self.channel4.volume_envelope.step();
            } else if self.time_divider & 1 == 1 {
                self.channel1.step_sweep();
            }

            self.time_divider = (self.time_divider + 1) % 4;
            self.prev_time = self.next_time;
            self.next_time += CLOCKS_PER_SECOND / 256;
        }

        if self.prev_time != self.time {
            self.channel1.run(self.prev_time, self.time);
            self.channel2.run(self.prev_time, self.time);
            self.channel3.run(self.prev_time, self.time);
            self.channel4.run(self.prev_time, self.time);

            self.prev_time = self.time;
        }
    }

    fn mix_buffers(&mut self) {
        let sample_count = self.channel1.blip.samples_avail() as usize;
        debug_assert!(sample_count == self.channel2.blip.samples_avail() as usize);
        debug_assert!(sample_count == self.channel3.blip.samples_avail() as usize);
        debug_assert!(sample_count == self.channel4.blip.samples_avail() as usize);

        let mut outputted = 0;

        let left_vol = (self.volume_left as f32 / 7.0) * (1.0 / 15.0) * 0.25;
        let right_vol = (self.volume_right as f32 / 7.0) * (1.0 / 15.0) * 0.25;

        while outputted < sample_count {
            let buf_left = &mut [0f32; OUTPUT_SAMPLE_COUNT + 10];
            let buf_right = &mut [0f32; OUTPUT_SAMPLE_COUNT + 10];
            let buf = &mut [0i16; OUTPUT_SAMPLE_COUNT + 10];

            let count1 = self.channel1.blip.read_samples(buf, false);
            for (i, v) in buf[..count1].iter().enumerate() {
                if self.register_data[0x15] & 0x01 == 0x01 {
                    buf_left[i] += *v as f32 * left_vol;
                }
                if self.register_data[0x15] & 0x10 == 0x10 {
                    buf_right[i] += *v as f32 * right_vol;
                }
            }

            let count2 = self.channel2.blip.read_samples(buf, false);
            for (i, v) in buf[..count2].iter().enumerate() {
                if self.register_data[0x15] & 0x02 == 0x02 {
                    buf_left[i] += *v as f32 * left_vol;
                }
                if self.register_data[0x15] & 0x20 == 0x20 {
                    buf_right[i] += *v as f32 * right_vol;
                }
            }

            // channel3 is the WaveChannel, that outputs samples with a 4x
            // increase in amplitude in order to avoid a loss of precision.
            let count3 = self.channel3.blip.read_samples(buf, false);
            for (i, v) in buf[..count3].iter().enumerate() {
                if self.register_data[0x15] & 0x04 == 0x04 {
                    buf_left[i] += ((*v as f32) / 4.0) * left_vol;
                }
                if self.register_data[0x15] & 0x40 == 0x40 {
                    buf_right[i] += ((*v as f32) / 4.0) * right_vol;
                }
            }

            let count4 = self.channel4.blip.read_samples(buf, false);
            for (i, v) in buf[..count4].iter().enumerate() {
                if self.register_data[0x15] & 0x08 == 0x08 {
                    buf_left[i] += *v as f32 * left_vol;
                }
                if self.register_data[0x15] & 0x80 == 0x80 {
                    buf_right[i] += *v as f32 * right_vol;
                }
            }

            debug_assert!(count1 == count2);
            debug_assert!(count1 == count3);
            debug_assert!(count1 == count4);

            self.player.play(&buf_left[..count1], &buf_right[..count1]);

            outputted += count1;
        }
    }

    fn clear_buffers(&mut self) {
        self.channel1.blip.clear();
        self.channel2.blip.clear();
        self.channel3.blip.clear();
        self.channel4.blip.clear();
    }
}

fn create_blipbuf(samples_rate: u32) -> BlipBuf {
    let mut blipbuf = BlipBuf::new(samples_rate);
    blipbuf.set_rates(CLOCKS_PER_SECOND as f64, samples_rate as f64);
    blipbuf
}
