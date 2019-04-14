from wasmer import Instance, Memory, Uint8Array, Int8Array, Uint16Array, Int16Array, Uint32Array, Int32Array
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

def test_get():
    memory = Instance(TEST_BYTES).memory.uint8_view()
    index = 7
    value = 42
    memory[index] = value

    assert memory[index] ==  value

def test_get_out_of_range():
    with pytest.raises(IndexError) as context_manager:
        memory = Instance(TEST_BYTES).memory.uint8_view()
        memory[len(memory) + 1]

    exception = context_manager.value
    assert str(exception) == (
        'Out of bound: Absolute index 1114113 is larger than the memory size 1114112.'
    )

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

    assert string, 'Hello ==  World!'

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
