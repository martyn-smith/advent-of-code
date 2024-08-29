"""
Testing utilities now that year 2020 is complete. Discoverable with pytest.
Naturally, if running with data other than mine, different answers would be expected to pass.
"""


def test_day_1():
    import aoc1

    assert aoc1.part_1() == 224436
    assert aoc1.part_2() == 303394260


def test_day_2():
    import aoc2

    assert aoc2.part_1() == 638
    assert aoc2.part_2() == 699


def test_day_3():
    import aoc3

    assert aoc3.part_1() == 244
    assert aoc3.part_2() == 9406609920


def test_day_4():
    import aoc4

    assert aoc4.part_1() == 213
    assert aoc4.part_2() == 147


def test_day_5():
    import aoc5

    assert aoc5.part_1() == 801
    assert aoc5.part_2() == 597


def test_day_6():
    import aoc6

    assert aoc6.part_1() == 6549
    assert aoc6.part_2() == 3466


def test_day_7():
    import aoc7

    assert aoc7.part_1() == 326
    assert aoc7.part_2() == 5635


def test_day_8():
    import aoc8

    assert aoc8.part_1() == 1331
    assert aoc8.part_2() == 1121


def test_day_9():
    import aoc9

    assert aoc9.part_1() == 393911906
    assert aoc9.part_2() == 59341885


def test_day_10():
    import aoc10

    assert aoc10.part_1() == 2376
    assert aoc10.part_2() == 129586085429248


def test_day_11():
    import aoc11

    assert aoc11.part_1() == 2321
    assert aoc11.part_2() == 2102


def test_day_12():
    import aoc12

    assert aoc12.part_1() == 319
    assert aoc12.part_2() == 50157


def test_day_13():
    import aoc13

    assert aoc13.part_1() == 4722
    assert aoc13.part_2() == 825305207525452


def test_day_14():
    import aoc14

    assert aoc14.part_1() == 10452688630537
    assert aoc14.part_2() == 2881082759597


def test_day_15():
    import aoc15

    assert aoc15.part_1() == 206
    assert aoc15.part_2() == 955


def test_day_16():
    import aoc16

    assert aoc16.part_1() == 24021
    assert aoc16.part_2() == 1289178686687


def test_day_17():
    import aoc17

    assert aoc17.part_1() == 240
    assert aoc17.part_2() == 1180


def test_day_18():
    import aoc18

    assert aoc18.part_1() == 50956598240016
    assert aoc18.part_2() == 535809575344339


def test_day_19():
    import aoc19

    assert aoc19.part_1() == 213
    assert aoc19.part_2() == 325


def test_day_20():
    import aoc20

    assert aoc20.part_1() == 108603771107737
    assert aoc20.part_2() == 2129


def test_day_21():
    import aoc21

    assert aoc21.part_1() == 2412
    assert aoc21.part_2() == "mfp,mgvfmvp,nhdjth,hcdchl,dvkbjh,dcvrf,bcjz,mhnrqp"


def test_day_22():
    import aoc22

    assert aoc22.part_1() == 32783
    assert aoc22.part_2() == 33455


def test_day_23():
    import aoc23

    assert aoc23.part_1() == 52937846
    assert aoc23.part_2() == 8456532414


def test_day_24():
    import aoc24

    assert aoc24.part_1() == 438
    assert aoc24.part_2() == 4038


def test_day_25():
    import aoc25

    assert aoc25.final_part() == 10548634
