# written for  python 3.10
import segno
from bitarray import bitarray
import sys

def draw_scaled_line(bits, scale):
    px_white = "█"
    px_black = " "
    return [px_white*scale if bit else px_black*scale for bit in bits]

def get_scaling(length):
    return 128 // length

def get_border_px(length):
    val = 128 % length // 2
    if 128 % length % 2 == 0:
        return (val, val)
    else:
        return (val, val+1)

def get_border_bits(length):
    zero_bit = bitarray("0")
    before, after = get_border_px(length)
    return before*zero_bit, after*zero_bit
    
def save_qr(str_in):
    qr_code = segno.make_qr(str_in)
    qr_code.save(
        "qr_new.pbm",
        scale=4,
        border=1,
    )
def extend_qr_array(bit_matrix):
    if not len(bit_matrix) == len(bit_matrix[1]):
        raise ValueError("generated invalid QR Code: not square")
    if len(bit_matrix) > 128:
        raise ValueError("generated invalid QR Code: too big")

    size = len(bit_matrix)
    left_border_bits, right_border_bits = get_border_bits(size)
    scaling = get_scaling(size)
    matrix_out = []
    zero_row = bitarray(128*str(0))
    upper_border_len, lower_border_len = get_border_px(size)

    # add border on top
    for _ in range(upper_border_len):
        matrix_out.append(zero_row)

    # add scaled bit rows
    for bit_row in bit_matrix:
        array_out = bitarray(
                             left_border_bits
                             )
        for bit in bit_row:
            array_out.extend(bitarray(scaling*str(bit)))
        array_out.extend(right_border_bits)
        if len(array_out) == 128:
            for _ in range(scaling):
                matrix_out.append(array_out)
        else:
            raise ValueError("Failed QR Code scaling, width invalid")
    # add lower border
    for _ in range(lower_border_len):
        matrix_out.append(zero_row)
    if len(matrix_out) == 128:
        return matrix_out
    else:
        raise ValueError("Failed QR Code scaling, hight invalid")
    
if __name__ == "__main__":
    str_in = sys.argv[1]
    qr_code = segno.make_qr(str_in)
    scaled_matrix = extend_qr_array(
        qr_code.matrix
    )
    with open("qr_bits.txt","w") as f:
        for line in scaled_matrix:
            text = "".join(draw_scaled_line(line,1))
            f.write(text)
            f.write("\n")
            
    with open("qr_bits.data", "wb") as q:
        for line in scaled_matrix:
            q.write(line)
    print("Saved qr_bits.data")
