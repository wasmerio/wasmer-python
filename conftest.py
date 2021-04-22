import enum
import inspect


def pytest_generate_tests(metafunc):
    if 'docexample' in metafunc.fixturenames:
        import wasmer
        docexamples = list(collect_doc_tests(wasmer))
        metafunc.parametrize("docexample", docexamples)


def collect_doc_tests(root):
    for obj, doc in collect_docs(root):
        for start_line, source in parse_doc_string(doc):
            yield DocTest(obj, source, start_line)


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
    start_line_idx = None
    current = []

    for line_idx, line in enumerate(doc.splitlines()):
        if state is ParserState.WAIT_SOURCE and line.strip() == "```py":
            state = ParserState.IN_SOURCE
            start_line_idx = line_idx

        elif state is ParserState.IN_SOURCE and line.strip() != "```":
            current.append(line)

        elif state is ParserState.IN_SOURCE and line.strip() == "```":
            yield start_line_idx, "\n".join(current)

            current = []
            state = ParserState.WAIT_SOURCE


class ParserState(int, enum.Enum):
    WAIT_SOURCE = enum.auto()
    IN_SOURCE = enum.auto()


class DocTest:
    def __init__(self, obj, source, start_line):
        self.obj = obj
        self.source = source
        self.start_line = start_line

    def exec(self):
        # make sure the line numbers between exception and  source agree
        source = "\n" * self.start_line + self.source

        try:
            co = compile(source, filename="<doc>", mode="exec")
            exec(co)

        except Exception as cause:
            raise DocTestError(self.obj, self.source, cause) from cause


class DocTestError(Exception):
    def __str__(self):
        obj, source, cause = self.args

        lineno = cause.__traceback__.tb_lineno

        return "Error in docstring of {!r} in line {}: {}\n{}".format(obj, lineno, cause, source)
