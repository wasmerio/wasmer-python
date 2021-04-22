import enum
import inspect


def pytest_generate_tests(metafunc):
    if 'docexample' in metafunc.fixturenames:
        import wasmer
        docexamples = list(collect_doc_tests(wasmer))
        metafunc.parametrize("docexample", docexamples)

    
def collect_doc_tests(root):
    for obj, doc in collect_docs(root):
        for source in parse_doc_string(doc):
            yield DocTest(obj, source)


def collect_docs(root):
    if hasattr(root, "__doc__") and root.__doc__ is not None:
        yield root, root.__doc__
    
    if inspect.ismodule(root) or inspect.isclass(root):
        for name, obj in vars(root).items():
            if name.startswith("_"):
                continue
            
            yield from collect_docs(obj)
            

def parse_doc_string(doc):
    state = ParserState.WAIT_SOURCE
    current = []
    
    for line in doc.splitlines():
        if state is ParserState.WAIT_SOURCE and line.strip() == "```py":
            state = ParserState.IN_SOURCE
        
        elif state is ParserState.IN_SOURCE and line.strip() != "```":
            current.append(line)
        
        elif state is ParserState.IN_SOURCE and line.strip() == "```":
            yield "\n".join(current)
            
            current = []
            state = ParserState.WAIT_SOURCE
    

class ParserState(int, enum.Enum):
    WAIT_SOURCE = enum.auto()
    IN_SOURCE = enum.auto()


class DocTest:
    def __init__(self, obj, source):
        self.obj = obj
        self.source = source

    def exec(self):
        co = compile(self.source, filename="foo", mode="exec")
        exec(co)
