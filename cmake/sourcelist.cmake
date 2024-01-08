set(headers ${headers}
    src/PCH.h
    src/skse/cosave.h
    src/skse/hooks.h
    src/skse/papyrus.h
    src/skse/sinks.h
    src/util/log.h
    src/util/util.h
)
set(sources ${sources}
    ${headers}
    src/main.cpp
    src/skse/cosave.cpp
    src/skse/hooks.cpp
    src/skse/papyrus.cpp
    src/skse/sinks.cpp
    src/util/util.cpp
)
