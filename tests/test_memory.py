from wasmer import Instance, Memory, Uint8Array, Int8Array, Uint16Array, Int16Array, Uint32Array, Int32Array, Buffer
import inspect
import os
import pytest

here = os.path.dirname(os.path.realpath(__file__))
TEST_BYTES = open(here + '/tests.wasm', 'rb').read()

def test_is_a_class():
    assert inspect.isclass(Memory)
    assert inspect.isclass(Uint8Array)
    assert inspect.isclass(Int8Array)
    assert inspect.isclass(Uint16Array)
    assert inspect.isclass(Int16Array)
    assert inspect.isclass(Uint32Array)
    assert inspect.isclass(Int32Array)

def test_bytes_per_element():
    assert Instance(TEST_BYTES).memory.uint8_view().bytes_per_element ==  1
    assert Instance(TEST_BYTES).memory.int8_view().bytes_per_element ==  1
    assert Instance(TEST_BYTES).memory.uint16_view().bytes_per_element ==  2
    assert Instance(TEST_BYTES).memory.int16_view().bytes_per_element ==  2
    assert Instance(TEST_BYTES).memory.uint32_view().bytes_per_element ==  4
    assert Instance(TEST_BYTES).memory.int32_view().bytes_per_element ==  4

@pytest.mark.xfail()
def test_cannot_construct():
    assert isinstance(Uint8Array(0), Uint8Array)

def test_length():
    assert len(Instance(TEST_BYTES).memory.uint8_view()) == (
        1114112
    )

def test_get_index():
    memory = Instance(TEST_BYTES).memory.uint8_view()
    index = 7
    value = 42
    memory[index] = value

    assert memory[index] == value

def test_get_integer_out_of_range_too_large():
    with pytest.raises(IndexError) as context_manager:
        memory = Instance(TEST_BYTES).memory.uint8_view()
        memory[len(memory) + 1]

    exception = context_manager.value
    assert str(exception) == (
        'Out of bound: Maximum index 1114113 is larger than the memory size 1114112.'
    )

def test_get_integer_out_of_range_negative():
    with pytest.raises(IndexError) as context_manager:
        memory = Instance(TEST_BYTES).memory.uint8_view()
        memory[-1]

    exception = context_manager.value
    assert str(exception) == (
        'Out of bound: Index cannot be negative.'
    )

def test_get_slice():
    memory = Instance(TEST_BYTES).memory.uint8_view()
    index = 7
    memory[index    ] = 1
    memory[index + 1] = 2
    memory[index + 2] = 3

    assert memory[index:index + 3] == [1, 2, 3]

def test_get_slice_out_of_range_empty():
    with pytest.raises(IndexError) as context_manager:
        memory = Instance(TEST_BYTES).memory.uint8_view()
        memory[2:1]

    exception = context_manager.value
    assert str(exception) == (
        'Slice `2:1` cannot be empty.'
    )

def test_get_slice_out_of_range_invalid_step():
    with pytest.raises(IndexError) as context_manager:
        memory = Instance(TEST_BYTES).memory.uint8_view()
        memory[1:7:2]

    exception = context_manager.value
    assert str(exception) == (
        'Slice must have a step of 1 for now; given 2.'
    )

def test_get_invalid_index():
    with pytest.raises(ValueError) as context_manager:
        memory = Instance(TEST_BYTES).memory.uint8_view()
        memory['a']

    exception = context_manager.value
    assert str(exception) == (
        'Only integers and slices are valid to represent an index.'
    )

def test_set_single_value():
    memory = Instance(TEST_BYTES).memory.uint8_view()

    assert memory[7] == 0
    memory[7] = 42
    assert memory[7] == 42

def test_set_list():
    memory = Instance(TEST_BYTES).memory.uint8_view()

    memory[7:12] = [1, 2, 3, 4, 5]
    assert memory[7:12] == [1, 2, 3, 4, 5]

def test_set_bytes():
    memory = Instance(TEST_BYTES).memory.uint8_view()

    memory[7:12] = bytes(b'abcde')
    assert memory[7:12] == [97, 98, 99, 100, 101]

def test_set_bytearray():
    memory = Instance(TEST_BYTES).memory.uint8_view()

    memory[7:12] = bytearray(b'abcde')
    assert memory[7:12] == [97, 98, 99, 100, 101]

def test_set_values_with_slice_and_step():
    memory = Instance(TEST_BYTES).memory.uint8_view()

    memory[7:12:2] = [1, 2, 3, 4, 5]
    assert memory[7:12] == [1, 0, 2, 0, 3]
    
def test_set_out_of_range():
    with pytest.raises(IndexError) as context_manager:
        memory = Instance(TEST_BYTES).memory.uint8_view()
        memory[len(memory) + 1] = 42

    exception = context_manager.value
    assert str(exception) == (
        'Out of bound: Absolute index 1114113 is larger than the memory size 1114112.'
    )

def test_hello_world():
    instance = Instance(TEST_BYTES)
    pointer = instance.exports.string()
    memory = instance.memory.uint8_view(pointer)
    nth = 0
    string = ''

    while (0 != memory[nth]):
        string += chr(memory[nth])
        nth += 1

    assert string, 'Hello, World!'

def test_memory_views_share_the_same_buffer():
    instance = Instance(TEST_BYTES)
    int8 = instance.memory.int8_view()
    int16 = instance.memory.int16_view()
    int32 = instance.memory.int32_view()

    int8[0] = 0b00000001
    int8[1] = 0b00000100
    int8[2] = 0b00010000
    int8[3] = 0b01000000

    assert int8[0] == 0b00000001
    assert int8[1] == 0b00000100
    assert int8[2] == 0b00010000
    assert int8[3] == 0b01000000
    assert int16[0] == 0b00000100_00000001
    assert int16[1] == 0b01000000_00010000
    assert int32[0] == 0b01000000_00010000_00000100_00000001

def test_memory_grow():
    instance = Instance(TEST_BYTES)
    memory = instance.memory
    int8 = memory.int8_view()

    old_memory_length = len(int8)

    assert old_memory_length == 1114112

    memory.grow(1)

    memory_length = len(int8)

    assert memory_length == 1179648
    assert memory_length - old_memory_length == 65536

def test_memory_grow_too_much():
    with pytest.raises(RuntimeError) as context_manager:
        Instance(TEST_BYTES).memory.grow(100000)

    exception = context_manager.value
    assert str(exception) == (
        'Failed to grow the memory: Grow Error: Failed to add pages because would exceed maximum number of pages. Left: 17, Right: 100000, Pages added: 100017.'
    )

def test_memory_is_absent():
    bytes = open(here + '/no_memory.wasm', 'rb').read()
    instance = Instance(bytes)

    assert instance.memory == None

def test_memory_buffer():
    memory = Instance(TEST_BYTES).memory.buffer
    assert isinstance(memory, Buffer)

def test_memory_buffer_memoryview():
    memory = Instance(TEST_BYTES).memory

    int8 = memory.int8_view()
    int8[0] = 1
    int8[1] = 2
    int8[2] = 3

    memory_view = memoryview(memory.buffer)

    assert memory_view.nbytes == 1114112
    assert memory_view.readonly == True
    assert memory_view.format == 'B'
    assert memory_view.itemsize == 1
    assert memory_view.ndim == 1
    assert memory_view.shape == (1114112,)
    assert memory_view.strides == (1,)
    assert memory_view.suboffsets == ()
    assert memory_view.c_contiguous == True
    assert memory_view.f_contiguous == True
    assert memory_view.contiguous == True
    assert memory_view[0:3].tolist() == [1, 2, 3]

def test_memory_buffer_bytearray():
    memory = Instance(TEST_BYTES).memory

    int8 = memory.int8_view()
    int8[0] = 1
    int8[1] = 2
    int8[2] = 3

    byte_array = bytearray(memory.buffer)

    assert len(byte_array) == 1114112
    assert byte_array[0:3] == b'\x01\x02\x03'
