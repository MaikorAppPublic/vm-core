use crate::offset;
use crate::single::test_op_init;
use maikor_platform::ops::*;
use maikor_platform::registers::id;

#[test]
fn mcpy() {
    test_op_init(
        "MCPY $100 $200 10",
        &[MEM_CPY_ADDR_ADDR_BYTE, 0, 100, 0, 200, 10],
        &[],
        &[
            (200, 5),
            (202, 99),
            (207, 200),
            (208, 1),
            (209, 2),
            (210, 12),
            (211, 51),
        ],
        &[],
        &[(100, 5), (102, 99), (107, 200), (108, 1), (109, 2)],
    );

    test_op_init(
        "MCPY $100 $202 CH",
        &[MEM_CPY_ADDR_ADDR_REG, 0, 100, 0, 202, id::CH as u8],
        &[(offset::CH, 3)],
        &[
            (200, 5),
            (202, 99),
            (207, 200),
            (208, 1),
            (209, 2),
            (210, 12),
            (211, 51),
        ],
        &[],
        &[(100, 99)],
    );

    test_op_init(
        "MCPY $100 DX 12",
        &[MEM_CPY_ADDR_REG_BYTE, 0, 100, id::DX as u8, 4],
        &[(offset::DL, 99)],
        &[(98, 4), (99, 10), (100, 105), (101, 178)],
        &[],
        &[(100, 10), (101, 105), (102, 178)],
    );
}
