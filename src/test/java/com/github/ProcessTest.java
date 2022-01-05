package com.github;

import java.util.Optional;

public class ProcessTest {

    public static void main(String[] args) {
        //        ProcessHandle.allProcesses()
        //                .forEach(process -> System.out.println(processDetails(process)));

        ProcessHandle.allProcesses().forEach(x -> {
            ProcessHandle.Info info = x.info();
            Optional<String> commandLineOpt = info.commandLine();
            if (info.command().isPresent()) {
                String x1 = info.command().get();
                if (x1.contains("pythonProject\\main.dist\\main.exe")) {
//                    x.destroyForcibly();
                    System.out.println(x1);
                }
            }
            if (commandLineOpt.isPresent()) {
                String s = commandLineOpt.get();
                boolean main = s.contains("main");
                System.out.println(s);
            }
        });
    }

    private static String processDetails(ProcessHandle process) {
        return String.format("%8d %8s %10s %26s %-40s",
                process.pid(),
                text(process.parent().map(ProcessHandle::pid)),
                text(process.info().user()),
                text(process.info().startInstant()),
                text(process.info().commandLine()));
    }

    private static String text(Optional<?> optional) {
        return optional.map(Object::toString).orElse("-");
    }
}
