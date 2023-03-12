extern crate env_logger;
use log::{debug, info, trace};

// SINE ranges between 2^15 - 1 and - (2^15 -1) or 1<<15
static SINE: [i32; 2048] = [
    0, 100, 201, 301, 402, 502, 603, 703, 804, 904, 1005, 1105, 1206, 1306, 1406, 1507, 1607, 1708,
    1808, 1908, 2009, 2109, 2209, 2310, 2410, 2510, 2610, 2711, 2811, 2911, 3011, 3111, 3211, 3311,
    3411, 3511, 3611, 3711, 3811, 3911, 4011, 4110, 4210, 4310, 4409, 4509, 4608, 4708, 4807, 4907,
    5006, 5106, 5205, 5304, 5403, 5502, 5601, 5700, 5799, 5898, 5997, 6096, 6195, 6293, 6392, 6491,
    6589, 6688, 6786, 6884, 6982, 7081, 7179, 7277, 7375, 7473, 7571, 7668, 7766, 7864, 7961, 8059,
    8156, 8253, 8351, 8448, 8545, 8642, 8739, 8836, 8932, 9029, 9126, 9222, 9319, 9415, 9511, 9607,
    9703, 9799, 9895, 9991, 10087, 10182, 10278, 10373, 10469, 10564, 10659, 10754, 10849, 10944,
    11038, 11133, 11227, 11322, 11416, 11510, 11604, 11698, 11792, 11886, 11980, 12073, 12166,
    12260, 12353, 12446, 12539, 12632, 12724, 12817, 12909, 13002, 13094, 13186, 13278, 13370,
    13462, 13553, 13645, 13736, 13827, 13918, 14009, 14100, 14191, 14281, 14372, 14462, 14552,
    14642, 14732, 14822, 14911, 15001, 15090, 15179, 15268, 15357, 15446, 15534, 15623, 15711,
    15799, 15887, 15975, 16063, 16150, 16238, 16325, 16412, 16499, 16586, 16672, 16759, 16845,
    16931, 17017, 17103, 17189, 17274, 17360, 17445, 17530, 17615, 17699, 17784, 17868, 17952,
    18036, 18120, 18204, 18287, 18371, 18454, 18537, 18620, 18702, 18785, 18867, 18949, 19031,
    19113, 19194, 19276, 19357, 19438, 19519, 19599, 19680, 19760, 19840, 19920, 20000, 20079,
    20159, 20238, 20317, 20396, 20474, 20553, 20631, 20709, 20787, 20864, 20942, 21019, 21096,
    21173, 21249, 21326, 21402, 21478, 21554, 21629, 21705, 21780, 21855, 21930, 22004, 22079,
    22153, 22227, 22301, 22374, 22448, 22521, 22594, 22666, 22739, 22811, 22883, 22955, 23027,
    23098, 23169, 23240, 23311, 23382, 23452, 23522, 23592, 23661, 23731, 23800, 23869, 23938,
    24006, 24075, 24143, 24211, 24278, 24346, 24413, 24480, 24546, 24613, 24679, 24745, 24811,
    24877, 24942, 25007, 25072, 25136, 25201, 25265, 25329, 25392, 25456, 25519, 25582, 25645,
    25707, 25769, 25831, 25893, 25954, 26016, 26077, 26137, 26198, 26258, 26318, 26378, 26437,
    26497, 26556, 26615, 26673, 26731, 26789, 26847, 26905, 26962, 27019, 27076, 27132, 27188,
    27244, 27300, 27355, 27411, 27466, 27520, 27575, 27629, 27683, 27736, 27790, 27843, 27896,
    27948, 28001, 28053, 28105, 28156, 28208, 28259, 28309, 28360, 28410, 28460, 28510, 28559,
    28608, 28657, 28706, 28754, 28802, 28850, 28897, 28945, 28992, 29038, 29085, 29131, 29177,
    29222, 29268, 29313, 29358, 29402, 29446, 29490, 29534, 29577, 29621, 29663, 29706, 29748,
    29790, 29832, 29873, 29915, 29955, 29996, 30036, 30076, 30116, 30156, 30195, 30234, 30272,
    30311, 30349, 30386, 30424, 30461, 30498, 30535, 30571, 30607, 30643, 30678, 30713, 30748,
    30783, 30817, 30851, 30885, 30918, 30951, 30984, 31017, 31049, 31081, 31113, 31144, 31175,
    31206, 31236, 31267, 31297, 31326, 31356, 31385, 31413, 31442, 31470, 31498, 31525, 31553,
    31580, 31606, 31633, 31659, 31684, 31710, 31735, 31760, 31785, 31809, 31833, 31856, 31880,
    31903, 31926, 31948, 31970, 31992, 32014, 32035, 32056, 32077, 32097, 32117, 32137, 32156,
    32176, 32194, 32213, 32231, 32249, 32267, 32284, 32301, 32318, 32334, 32350, 32366, 32382,
    32397, 32412, 32426, 32441, 32455, 32468, 32482, 32495, 32508, 32520, 32532, 32544, 32556,
    32567, 32578, 32588, 32599, 32609, 32618, 32628, 32637, 32646, 32654, 32662, 32670, 32678,
    32685, 32692, 32699, 32705, 32711, 32717, 32722, 32727, 32732, 32736, 32740, 32744, 32748,
    32751, 32754, 32757, 32759, 32761, 32763, 32764, 32765, 32766, 32766, 32767, 32766, 32766,
    32765, 32764, 32763, 32761, 32759, 32757, 32754, 32751, 32748, 32744, 32740, 32736, 32732,
    32727, 32722, 32717, 32711, 32705, 32699, 32692, 32685, 32678, 32670, 32662, 32654, 32646,
    32637, 32628, 32618, 32609, 32599, 32588, 32578, 32567, 32556, 32544, 32532, 32520, 32508,
    32495, 32482, 32468, 32455, 32441, 32426, 32412, 32397, 32382, 32366, 32350, 32334, 32318,
    32301, 32284, 32267, 32249, 32231, 32213, 32194, 32176, 32156, 32137, 32117, 32097, 32077,
    32056, 32035, 32014, 31992, 31970, 31948, 31926, 31903, 31880, 31856, 31833, 31809, 31785,
    31760, 31735, 31710, 31684, 31659, 31633, 31606, 31580, 31553, 31525, 31498, 31470, 31442,
    31413, 31385, 31356, 31326, 31297, 31267, 31236, 31206, 31175, 31144, 31113, 31081, 31049,
    31017, 30984, 30951, 30918, 30885, 30851, 30817, 30783, 30748, 30713, 30678, 30643, 30607,
    30571, 30535, 30498, 30461, 30424, 30386, 30349, 30311, 30272, 30234, 30195, 30156, 30116,
    30076, 30036, 29996, 29955, 29915, 29873, 29832, 29790, 29748, 29706, 29663, 29621, 29577,
    29534, 29490, 29446, 29402, 29358, 29313, 29268, 29222, 29177, 29131, 29085, 29038, 28992,
    28945, 28897, 28850, 28802, 28754, 28706, 28657, 28608, 28559, 28510, 28460, 28410, 28360,
    28309, 28259, 28208, 28156, 28105, 28053, 28001, 27948, 27896, 27843, 27790, 27736, 27683,
    27629, 27575, 27520, 27466, 27411, 27355, 27300, 27244, 27188, 27132, 27076, 27019, 26962,
    26905, 26847, 26789, 26731, 26673, 26615, 26556, 26497, 26437, 26378, 26318, 26258, 26198,
    26137, 26077, 26016, 25954, 25893, 25831, 25769, 25707, 25645, 25582, 25519, 25456, 25392,
    25329, 25265, 25201, 25136, 25072, 25007, 24942, 24877, 24811, 24745, 24679, 24613, 24546,
    24480, 24413, 24346, 24278, 24211, 24143, 24075, 24006, 23938, 23869, 23800, 23731, 23661,
    23592, 23522, 23452, 23382, 23311, 23240, 23169, 23098, 23027, 22955, 22883, 22811, 22739,
    22666, 22594, 22521, 22448, 22374, 22301, 22227, 22153, 22079, 22004, 21930, 21855, 21780,
    21705, 21629, 21554, 21478, 21402, 21326, 21249, 21173, 21096, 21019, 20942, 20864, 20787,
    20709, 20631, 20553, 20474, 20396, 20317, 20238, 20159, 20079, 20000, 19920, 19840, 19760,
    19680, 19599, 19519, 19438, 19357, 19276, 19194, 19113, 19031, 18949, 18867, 18785, 18702,
    18620, 18537, 18454, 18371, 18287, 18204, 18120, 18036, 17952, 17868, 17784, 17699, 17615,
    17530, 17445, 17360, 17274, 17189, 17103, 17017, 16931, 16845, 16759, 16672, 16586, 16499,
    16412, 16325, 16238, 16150, 16063, 15975, 15887, 15799, 15711, 15623, 15534, 15446, 15357,
    15268, 15179, 15090, 15001, 14911, 14822, 14732, 14642, 14552, 14462, 14372, 14281, 14191,
    14100, 14009, 13918, 13827, 13736, 13645, 13553, 13462, 13370, 13278, 13186, 13094, 13002,
    12909, 12817, 12724, 12632, 12539, 12446, 12353, 12260, 12166, 12073, 11980, 11886, 11792,
    11698, 11604, 11510, 11416, 11322, 11227, 11133, 11038, 10944, 10849, 10754, 10659, 10564,
    10469, 10373, 10278, 10182, 10087, 9991, 9895, 9799, 9703, 9607, 9511, 9415, 9319, 9222, 9126,
    9029, 8932, 8836, 8739, 8642, 8545, 8448, 8351, 8253, 8156, 8059, 7961, 7864, 7766, 7668, 7571,
    7473, 7375, 7277, 7179, 7081, 6982, 6884, 6786, 6688, 6589, 6491, 6392, 6293, 6195, 6096, 5997,
    5898, 5799, 5700, 5601, 5502, 5403, 5304, 5205, 5106, 5006, 4907, 4807, 4708, 4608, 4509, 4409,
    4310, 4210, 4110, 4011, 3911, 3811, 3711, 3611, 3511, 3411, 3311, 3211, 3111, 3011, 2911, 2811,
    2711, 2610, 2510, 2410, 2310, 2209, 2109, 2009, 1908, 1808, 1708, 1607, 1507, 1406, 1306, 1206,
    1105, 1005, 904, 804, 703, 603, 502, 402, 301, 201, 100, 0, -100, -201, -301, -402, -502, -603,
    -703, -804, -904, -1005, -1105, -1206, -1306, -1406, -1507, -1607, -1708, -1808, -1908, -2009,
    -2109, -2209, -2310, -2410, -2510, -2610, -2711, -2811, -2911, -3011, -3111, -3211, -3311,
    -3411, -3511, -3611, -3711, -3811, -3911, -4011, -4110, -4210, -4310, -4409, -4509, -4608,
    -4708, -4807, -4907, -5006, -5106, -5205, -5304, -5403, -5502, -5601, -5700, -5799, -5898,
    -5997, -6096, -6195, -6293, -6392, -6491, -6589, -6688, -6786, -6884, -6982, -7081, -7179,
    -7277, -7375, -7473, -7571, -7668, -7766, -7864, -7961, -8059, -8156, -8253, -8351, -8448,
    -8545, -8642, -8739, -8836, -8932, -9029, -9126, -9222, -9319, -9415, -9511, -9607, -9703,
    -9799, -9895, -9991, -10087, -10182, -10278, -10373, -10469, -10564, -10659, -10754, -10849,
    -10944, -11038, -11133, -11227, -11322, -11416, -11510, -11604, -11698, -11792, -11886, -11980,
    -12073, -12166, -12260, -12353, -12446, -12539, -12632, -12724, -12817, -12909, -13002, -13094,
    -13186, -13278, -13370, -13462, -13553, -13645, -13736, -13827, -13918, -14009, -14100, -14191,
    -14281, -14372, -14462, -14552, -14642, -14732, -14822, -14911, -15001, -15090, -15179, -15268,
    -15357, -15446, -15534, -15623, -15711, -15799, -15887, -15975, -16063, -16150, -16238, -16325,
    -16412, -16499, -16586, -16672, -16759, -16845, -16931, -17017, -17103, -17189, -17274, -17360,
    -17445, -17530, -17615, -17699, -17784, -17868, -17952, -18036, -18120, -18204, -18287, -18371,
    -18454, -18537, -18620, -18702, -18785, -18867, -18949, -19031, -19113, -19194, -19276, -19357,
    -19438, -19519, -19599, -19680, -19760, -19840, -19920, -20000, -20079, -20159, -20238, -20317,
    -20396, -20474, -20553, -20631, -20709, -20787, -20864, -20942, -21019, -21096, -21173, -21249,
    -21326, -21402, -21478, -21554, -21629, -21705, -21780, -21855, -21930, -22004, -22079, -22153,
    -22227, -22301, -22374, -22448, -22521, -22594, -22666, -22739, -22811, -22883, -22955, -23027,
    -23098, -23169, -23240, -23311, -23382, -23452, -23522, -23592, -23661, -23731, -23800, -23869,
    -23938, -24006, -24075, -24143, -24211, -24278, -24346, -24413, -24480, -24546, -24613, -24679,
    -24745, -24811, -24877, -24942, -25007, -25072, -25136, -25201, -25265, -25329, -25392, -25456,
    -25519, -25582, -25645, -25707, -25769, -25831, -25893, -25954, -26016, -26077, -26137, -26198,
    -26258, -26318, -26378, -26437, -26497, -26556, -26615, -26673, -26731, -26789, -26847, -26905,
    -26962, -27019, -27076, -27132, -27188, -27244, -27300, -27355, -27411, -27466, -27520, -27575,
    -27629, -27683, -27736, -27790, -27843, -27896, -27948, -28001, -28053, -28105, -28156, -28208,
    -28259, -28309, -28360, -28410, -28460, -28510, -28559, -28608, -28657, -28706, -28754, -28802,
    -28850, -28897, -28945, -28992, -29038, -29085, -29131, -29177, -29222, -29268, -29313, -29358,
    -29402, -29446, -29490, -29534, -29577, -29621, -29663, -29706, -29748, -29790, -29832, -29873,
    -29915, -29955, -29996, -30036, -30076, -30116, -30156, -30195, -30234, -30272, -30311, -30349,
    -30386, -30424, -30461, -30498, -30535, -30571, -30607, -30643, -30678, -30713, -30748, -30783,
    -30817, -30851, -30885, -30918, -30951, -30984, -31017, -31049, -31081, -31113, -31144, -31175,
    -31206, -31236, -31267, -31297, -31326, -31356, -31385, -31413, -31442, -31470, -31498, -31525,
    -31553, -31580, -31606, -31633, -31659, -31684, -31710, -31735, -31760, -31785, -31809, -31833,
    -31856, -31880, -31903, -31926, -31948, -31970, -31992, -32014, -32035, -32056, -32077, -32097,
    -32117, -32137, -32156, -32176, -32194, -32213, -32231, -32249, -32267, -32284, -32301, -32318,
    -32334, -32350, -32366, -32382, -32397, -32412, -32426, -32441, -32455, -32468, -32482, -32495,
    -32508, -32520, -32532, -32544, -32556, -32567, -32578, -32588, -32599, -32609, -32618, -32628,
    -32637, -32646, -32654, -32662, -32670, -32678, -32685, -32692, -32699, -32705, -32711, -32717,
    -32722, -32727, -32732, -32736, -32740, -32744, -32748, -32751, -32754, -32757, -32759, -32761,
    -32763, -32764, -32765, -32766, -32766, -32767, -32766, -32766, -32765, -32764, -32763, -32761,
    -32759, -32757, -32754, -32751, -32748, -32744, -32740, -32736, -32732, -32727, -32722, -32717,
    -32711, -32705, -32699, -32692, -32685, -32678, -32670, -32662, -32654, -32646, -32637, -32628,
    -32618, -32609, -32599, -32588, -32578, -32567, -32556, -32544, -32532, -32520, -32508, -32495,
    -32482, -32468, -32455, -32441, -32426, -32412, -32397, -32382, -32366, -32350, -32334, -32318,
    -32301, -32284, -32267, -32249, -32231, -32213, -32194, -32176, -32156, -32137, -32117, -32097,
    -32077, -32056, -32035, -32014, -31992, -31970, -31948, -31926, -31903, -31880, -31856, -31833,
    -31809, -31785, -31760, -31735, -31710, -31684, -31659, -31633, -31606, -31580, -31553, -31525,
    -31498, -31470, -31442, -31413, -31385, -31356, -31326, -31297, -31267, -31236, -31206, -31175,
    -31144, -31113, -31081, -31049, -31017, -30984, -30951, -30918, -30885, -30851, -30817, -30783,
    -30748, -30713, -30678, -30643, -30607, -30571, -30535, -30498, -30461, -30424, -30386, -30349,
    -30311, -30272, -30234, -30195, -30156, -30116, -30076, -30036, -29996, -29955, -29915, -29873,
    -29832, -29790, -29748, -29706, -29663, -29621, -29577, -29534, -29490, -29446, -29402, -29358,
    -29313, -29268, -29222, -29177, -29131, -29085, -29038, -28992, -28945, -28897, -28850, -28802,
    -28754, -28706, -28657, -28608, -28559, -28510, -28460, -28410, -28360, -28309, -28259, -28208,
    -28156, -28105, -28053, -28001, -27948, -27896, -27843, -27790, -27736, -27683, -27629, -27575,
    -27520, -27466, -27411, -27355, -27300, -27244, -27188, -27132, -27076, -27019, -26962, -26905,
    -26847, -26789, -26731, -26673, -26615, -26556, -26497, -26437, -26378, -26318, -26258, -26198,
    -26137, -26077, -26016, -25954, -25893, -25831, -25769, -25707, -25645, -25582, -25519, -25456,
    -25392, -25329, -25265, -25201, -25136, -25072, -25007, -24942, -24877, -24811, -24745, -24679,
    -24613, -24546, -24480, -24413, -24346, -24278, -24211, -24143, -24075, -24006, -23938, -23869,
    -23800, -23731, -23661, -23592, -23522, -23452, -23382, -23311, -23240, -23169, -23098, -23027,
    -22955, -22883, -22811, -22739, -22666, -22594, -22521, -22448, -22374, -22301, -22227, -22153,
    -22079, -22004, -21930, -21855, -21780, -21705, -21629, -21554, -21478, -21402, -21326, -21249,
    -21173, -21096, -21019, -20942, -20864, -20787, -20709, -20631, -20553, -20474, -20396, -20317,
    -20238, -20159, -20079, -20000, -19920, -19840, -19760, -19680, -19599, -19519, -19438, -19357,
    -19276, -19194, -19113, -19031, -18949, -18867, -18785, -18702, -18620, -18537, -18454, -18371,
    -18287, -18204, -18120, -18036, -17952, -17868, -17784, -17699, -17615, -17530, -17445, -17360,
    -17274, -17189, -17103, -17017, -16931, -16845, -16759, -16672, -16586, -16499, -16412, -16325,
    -16238, -16150, -16063, -15975, -15887, -15799, -15711, -15623, -15534, -15446, -15357, -15268,
    -15179, -15090, -15001, -14911, -14822, -14732, -14642, -14552, -14462, -14372, -14281, -14191,
    -14100, -14009, -13918, -13827, -13736, -13645, -13553, -13462, -13370, -13278, -13186, -13094,
    -13002, -12909, -12817, -12724, -12632, -12539, -12446, -12353, -12260, -12166, -12073, -11980,
    -11886, -11792, -11698, -11604, -11510, -11416, -11322, -11227, -11133, -11038, -10944, -10849,
    -10754, -10659, -10564, -10469, -10373, -10278, -10182, -10087, -9991, -9895, -9799, -9703,
    -9607, -9511, -9415, -9319, -9222, -9126, -9029, -8932, -8836, -8739, -8642, -8545, -8448,
    -8351, -8253, -8156, -8059, -7961, -7864, -7766, -7668, -7571, -7473, -7375, -7277, -7179,
    -7081, -6982, -6884, -6786, -6688, -6589, -6491, -6392, -6293, -6195, -6096, -5997, -5898,
    -5799, -5700, -5601, -5502, -5403, -5304, -5205, -5106, -5006, -4907, -4807, -4708, -4608,
    -4509, -4409, -4310, -4210, -4110, -4011, -3911, -3811, -3711, -3611, -3511, -3411, -3311,
    -3211, -3111, -3011, -2911, -2811, -2711, -2610, -2510, -2410, -2310, -2209, -2109, -2009,
    -1908, -1808, -1708, -1607, -1507, -1406, -1306, -1206, -1105, -1005, -904, -804, -703, -603,
    -502, -402, -301, -201, -100,
];

// Simple complex data type, (simpler than num::complex::Complex)
#[derive(Copy, Clone)]
pub struct Complex {
    pub re: i32,
    pub im: i32,
}

impl Complex {
    // Create a new complex number
    pub fn new(re: i32, im: i32) -> Complex {
        // Initiate and return complex number
        Complex { re: re, im: im }
    }
    pub fn bitshift_right(&mut self, n: usize) {
        // Bitshifts real and imaginary parts to the right by n bits
        self.re >>= n;
        self.im >>= n;
    }
    // TODO: thoroughly check this method
    pub fn get_clipped_msb(self, nbits_keep: usize) -> Complex {
        // Clips most significant bits so that only nbits_keep least
        // significant bits are kept *in total*, that means we *assume*
        // that nbits_keep is even and we keep only nbits_keep/2 real
        // and imaginary bits.
        // nbits_keep/2 - 1, minus one is to account for the sign bit
        // The minus one after the bitshift is to turn 1000 into 0111
        let mask = (1 << nbits_keep / 2 - 1) - 1;
        let mut re: i32 = self.re & mask;
        let mut im: i32 = self.im & mask;
        if self.re < 0 {
            re = self.re & (-mask)
        };
        if self.im < 0 {
            im = self.im & (-mask)
        };
        Self::new(re, im)
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.im >= 0 {
            write!(f, "{}+{}i", self.re, self.im)
        } else {
            write!(f, "{}{}i", self.re, self.im)
        }
    }
}

impl std::ops::Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex {
        Complex::new(rhs.re + self.re, rhs.im + self.im)
    }
}

impl std::ops::Sub for Complex {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Complex {
        Complex::new(rhs.re - self.re, rhs.im - self.im)
    }
}

impl std::ops::Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Complex {
        Complex::new(
            rhs.re * self.re - rhs.im * self.im,
            rhs.re * self.im + rhs.im * self.re,
        )
    }
}

impl PartialEq for Complex {
    fn eq(&self, rhs: &Self) -> bool {
        self.re == rhs.re && self.im == rhs.im
    }
}

// copy a complex array from a into b
fn copy_ab(a: &[Complex], b: &mut [Complex]) {
    let len = a.len();
    assert!(len == b.len()); // check they are same size
    for idx in 0..len {
        b[idx] = a[idx];
    }
}

fn trace_array(arr: &[Complex]) {
    for c in arr.iter() {
        trace!("{}, ", c);
    }
}

fn display_array(arr: &[Complex]) {
    print!("[");
    for c in arr.iter() {
        print!("{}, ", c);
    }
    println!("]");
}

// Simple three-stage DFT Radix-2 DIT
#[allow(dead_code)]
fn fft8(flip: &mut [Complex; 8], flop: &mut [Complex; 8]) {
    // Decimation in time re-ordering, flip -> flop
    for idx in 0usize..8 {
        // Bit flipped idx
        let mut bfi: usize = 0;
        for pos in 0u8..=2 {
            bfi |= ((idx & (1 << pos)) >> pos) << (2 - pos);
        }
        trace!("idx={}, bfi={}", idx, bfi);
        // Copy the number at idx into bit-flipped-index
        flop[bfi] = flip[idx];
    }
    trace!("Bit-switch complete, result for flop:");
    trace_array(&*flop);
    trace!("Flip is:");
    trace_array(&*flip);
    // For each stage, compute the twiddle factors
    let mut size: usize; // size of the current butterfly stage
    let mut numb: usize; // number of chunks of size 'size', numb*size=8
    for stage in 1..=3 {
        // Copy flop back into flip
        copy_ab(&*flop, flip);
        trace!("\nButterfly stage {}", stage);
        size = 1 << stage;
        numb = 1 << (3 - stage);
        for chunk in 0..numb {
            for k in 0..(size / 2) {
                let mut d1 = flip[chunk * size + k];
                let twiddle = Complex::new(
                    SINE[2048 / 4 + numb * k * 2048 / 8],
                    SINE[numb * k * 2048 / 8],
                );
                let mut d2 = flip[chunk * size + size / 2 + k] * twiddle;
                d2.bitshift_right(15); // normalize, twiddle factor too big
                                       // bitshift right by 1 prevent Butterfly overflow
                d1.bitshift_right(1);
                d2.bitshift_right(1);
                flop[chunk * size + k] = d1 + d2;
                flop[chunk * size + k + size / 2] = d1 - d2;
            }
            trace!("\nchunk={}", chunk);
            display_array(&*flop);
        }
    }
}

#[allow(dead_code)]
fn fft2048(flip: &mut [Complex; 2048], flop: &mut [Complex; 2048]) {
    // Decimation in time re-ordering, flip -> flop
    for idx in 0usize..2048 {
        // Bit flipped idx
        let mut bfi: usize = 0;
        for pos in 0..=10 {
            bfi |= ((idx & (1 << pos)) >> pos) << (10 - pos);
        }
        trace!("idx={}, bfi={}", idx, bfi);
        // Copy the number at idx into bit-flipped-index
        flop[bfi] = flip[idx];
    }
    // For each stage, compute the twiddle factors
    let mut size: usize; // size of the current butterfly stage
    let mut numb: usize; // number of chunks of size 'size', numb*size=8
    for stage in 1u32..=11 {
        // Copy flop back into flip
        copy_ab(&*flop, flip);
        trace!("\nButterfly stage #{}", stage);
        size = 1 << stage;
        numb = 1 << (11 - stage);
        for chunk in 0usize..numb {
            for k in 0usize..(size / 2) {
                let mut d1 = flip[chunk * size + k];
                let twiddle = Complex::new(SINE[2048 / 4 + numb * k], SINE[numb * k]);
                let mut d2 = flip[chunk * size + size / 2 + k] * twiddle;
                d2.bitshift_right(15); // normalize, twiddle factor is order 2^15
                                       // bitshift right by 1 prevent Butterfly overflow
                d1.bitshift_right(1);
                d2.bitshift_right(1);
                flop[chunk * size + k] = d1 + d2;
                flop[chunk * size + k + size / 2] = d1 - d2;
            }
        }
    }
}

// FFT for any power of two up to and including 2048
fn fft(flip: &mut [Complex], flop: &mut [Complex]) {
    // Make sure length of arrays are a power of two
    let len = flip.len();
    assert!(len == flop.len());
    // init n: the logarithm of len, such that 2^n == len
    let mut n: usize = 0;
    // Loop to figure out what power of 2 len is and to check it is one
    for i in 0..=12 {
        n = i;
        if len == (1 << n) {
            break; // break the loop at the right power of two
        }
    }
    assert!(
        n < 12,
        "The length of our FFT must be a POWER OF TWO STRICTLY LESS than 12"
    );
    trace!("n = {}", n);
    // Decimation in time re-ordering, flip -> flop
    for idx in 0..len {
        // Bit Flipped Index
        let mut bfi: usize = 0;
        for pos in 0..n {
            bfi |= ((idx & (1 << pos)) >> pos) << (n - 1 - pos);
        }
        trace!("idx={}, bfi={}", idx, bfi);
        // Copy the number at index idx into Bit-Flipped-Index (bfi)
        flop[bfi] = flip[idx];
    }
    trace!("Bit-swich complete; result for flop:");
    trace_array(&*flop);
    trace!("Flip is:");
    trace_array(&*flip);
    // For each stage, compute the twiddle factors
    let mut size: usize; // size of the current butterfly stage
    let mut numb: usize; // number of chunks of size 'size', numb*size=len
    for stage in 1..=n {
        // Copy flop back into flip
        copy_ab(&*flop, flip);
        trace!("\nButterfly stage #{}", stage);
        size = 1 << stage;
        numb = 1 << (n - stage);
        for chunk in 0..numb {
            for k in 0..(size / 2) {
                let mut d1 = flip[chunk * size + k];
                let twiddle = Complex::new(
                    SINE[2048 / 4 + numb * k * (2048 >> n)],
                    SINE[numb * k * (2048 >> n)],
                );
                let mut d2 = flip[chunk * size + size / 2 + k] * twiddle;
                // normalize, twiddle factor is order 2^15
                d2.bitshift_right(15);
                // bitshift right by 1 prevent Butterfly overflow
                d1.bitshift_right(1);
                d2.bitshift_right(1);
                flop[chunk * size + k] = d1 + d2;
                flop[chunk * size + k + size / 2] = d1 - d2;
            }
        }
    }
}

// Note on design choice: we always MANIPULATE data going from flip to
// flop, and then COPY data back from flop into flip. This is not the
// fastest way to do an FFT, but it makes for readable code.
fn fft_quantized(
    flip: &mut [Complex], // input (also gets modified)
    flop: &mut [Complex], // output
    nsinebits: usize,     // number of bits used to store sine coeffs
    ndatabits: usize,     // number of bits used to store our data
    ntotalbits: usize,    // sinebits + databite, total num of bits used
) {
    trace!("Commencing basic tests and checks");
    assert_eq!(nsinebits + ndatabits, ntotalbits);
    // Our SINE lookup table is in i16, values in -2^15 to 2^15
    assert!(nsinebits <= 16);
    // ndatabits should be even, half for imaginary half for real
    assert!(ndatabits & 1 == 0);
    // Make sure length of arrays are a power of two
    let len = flip.len();
    assert!(len == flop.len());
    // initiate n: the log2 of len
    let mut n: usize = 0;
    // Loop to find what power of 2 len is, and check it really is one
    for i in 0..=12 {
        n = i;
        if len == (1 << n) {
            break; // break the loop at the correct power of two
        }
    }
    assert!(
        n < 12,
        "The length of our FFT must be a POWER OF TWO STRICTLY LESS than 12"
    );
    trace!("n = {}", n);
    trace!(
        "Clipping input data (flip) to {} bits to avoid overflow",
        ndatabits
    );
    for i in 0..len {
        flop[i] = flip[i].get_clipped_msb(ndatabits);
        if flop[i] != flip[i] {
            trace!("{} got clipped to {}", flop[i], flip[i]);
        }
    }
    // copy flop back into flip
    copy_ab(&*flop, flip);
    // TODO: refactor this for loop into a function used by all fft methods
    debug!("Starting Decimation In Time re-ordering");
    for idx in 0..len {
        // Bit Flipped Index
        let mut bfi: usize = 0;
        for pos in 0..n {
            bfi |= ((idx & (1 << pos)) >> pos) << (n - 1 - pos);
        }
        trace!("idx={}, bfi={}", idx, bfi);
        // copy the number at index idx into Bit-Flipped-Index (bfi)
        flop[bfi] = flip[idx];
    }
    trace!("Bit swich complete");
    // For each stage of FFT, compute the twiddle factors and multiply
    let mut size: usize; // size of the current butterfly stage
    let mut numb: usize; // number of chunks of size 'size', numb*size=len
    for stage in 1..=n {
        // Copy flop back into flip
        copy_ab(&*flop, flip);
        trace!("\nButterfly stage #{}", stage);
        size = 1 << stage;
        numb = 1 << (n - stage);
        for chunk in 0..numb {
            for k in 0..(size / 2) {
                let mut d1 = flip[chunk * size + k];
                let twiddle = Complex::new(
                    SINE[2048 / 4 + numb * k * (2048 >> n)] >> (16 - nsinebits),
                    SINE[numb * k * (2048 >> n)] >> (16 - nsinebits),
                );
                let mut d2 = flip[chunk * size + size / 2 + k] * twiddle;
                // normalize, twiddle factor is order 2^(nsinebits - 1)
                d2.bitshift_right(nsinebits - 1);
                // bitshift right by 1 prevent Butterfly overflow
                d1.bitshift_right(1);
                d2.bitshift_right(1);
                // Set next stage butterfly values
                flop[chunk * size + k] = d1 + d2;
                flop[chunk * size + k + size / 2] = d1 - d2;
            }
        }
    }
}

fn main() {
    env_logger::init();
    info!("Initiating array of 8 complex numbers");
    let mut flip: [Complex; 8] = [Complex::new(1000, 0); 8]; // input
    let mut flop: [Complex; 8] = [Complex::new(0, 0); 8]; // output
    info!("Performing FFT");
    fft(&mut flip, &mut flop);
    display_array(&flop); // Display result

    //info!("Initiating array of 2048 complex numbers");
    //let mut flip: [Complex; 2048] = [Complex::new(1_000, 0); 2048];
    //let mut flop: [Complex; 2048] = [Complex::new(0, 0); 2048];
    //info!("Performing FFT");
    //fft(&mut flip, &mut flop);
    //display_array(&flop); // Display result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_complex() {
        let _c = Complex::new(3, 1);
    }

    #[test]
    fn test_fft() {
        // Perform an fft, see if it breaks
        let mut flip: [Complex; 8] = [Complex::new(1000, 0); 8];
        let mut flop: [Complex; 8] = [Complex::new(1000, 0); 8];
        fft(&mut flip, &mut flop);

        // Compare output with output of fft8, should be exact same
        let mut flip8: [Complex; 8] = [Complex::new(1000, 0); 8];
        let mut flop8: [Complex; 8] = [Complex::new(1000, 0); 8];
        fft8(&mut flip8, &mut flop8);
        for i in 0..8 {
            assert!(flip[i] == flip8[i]);
            assert!(flop[i] == flop8[i]);
        }

        //// Compare output with fft2048, should be exact same
        // Perform 2048 point FFT with fft2048()
        let mut flip2048: [Complex; 2048] = [Complex::new(1_000, 0); 2048];
        let mut flop2048: [Complex; 2048] = [Complex::new(0, 0); 2048];
        fft2048(&mut flip2048, &mut flop2048);
        // Perform 2048 point FFT with fft()
        let mut flip: [Complex; 2048] = [Complex::new(1_000, 0); 2048];
        let mut flop: [Complex; 2048] = [Complex::new(0, 0); 2048];
        fft(&mut flip, &mut flop);
        for i in 0..2048 {
            assert!(flip[i] == flip2048[i]);
            assert!(flop[i] == flop2048[i]);
        }
    }

    #[test]
    fn test_fft_quantized() {
        // Perform an fft_quantized, see if it breaks
        let mut flip: [Complex; 8] = [Complex::new(100, 0); 8];
        let mut flop: [Complex; 8] = [Complex::new(100, 0); 8];
        fft_quantized(&mut flip, &mut flop, 16, 16, 32);
        // Compare output with other integer FFT
        let mut flip2: [Complex; 8] = [Complex::new(100, 0); 8];
        let mut flop2: [Complex; 8] = [Complex::new(100, 0); 8];
        fft(&mut flip2, &mut flop2);
        for i in 0..8 {
            assert!(flip[i] == flip2[i]);
            assert!(flop[i] == flop2[i]);
        }
    }
}
