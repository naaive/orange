package com.github;

import org.junit.jupiter.api.Test;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.Arrays;
import java.util.Optional;
import java.util.stream.Collectors;

public class ProcessTest {

    public static String getProcessOutput(String java) throws IOException, InterruptedException
    {
        ProcessBuilder processBuilder = new ProcessBuilder(java);

        processBuilder.redirectErrorStream(true);

        Process process = processBuilder.start();
        StringBuilder processOutput = new StringBuilder();

        try (BufferedReader processOutputReader = new BufferedReader(
                new InputStreamReader(process.getInputStream()));)
        {
            String readLine;

            while ((readLine = processOutputReader.readLine()) != null)
            {
                processOutput.append(readLine + System.lineSeparator());
            }

            process.waitFor();
        }

        return processOutput.toString().trim();
    }
    @Test
    void cpu() throws IOException, InterruptedException {

//        BufferedReader errorReader = new BufferedReader(new InputStreamReader(process.getErrorStream()));
//        errorReader.lines().forEach(System.out::println);
    }

    public static void main(String[] args) {
        //        ProcessHandle.allProcesses()
        //                .forEach(process -> System.out.println(processDetails(process)));

        ProcessHandle.allProcesses().forEach(x -> {
            ProcessHandle.Info info = x.info();
            Optional<String> commandLineOpt = info.commandLine();
            String orange_core = "orange";
//            String orange_core = "lib";
            if (info.command().isPresent()) {
                String x1 = info.command().get();
                if (x1.contains(orange_core)) {
//                    x.destroyForcibly();
                    System.out.println(x1);
                }
            }
            if (commandLineOpt.isPresent()) {
                String s = commandLineOpt.get();
                boolean main = s.contains(orange_core);
                System.out.println(s);
            }
        });
    }

    @Test
    void name() {
        killByPort(3000);
    }

    private void killByPort   (int port) {
        try{
            Runtime rt = Runtime.getRuntime();
            Process proc = rt.exec("cmd /c netstat -ano | findstr " + port);

            BufferedReader stdInput = new BufferedReader(new
                    InputStreamReader(proc.getInputStream()));
            String s = null;
            if ((s = stdInput.readLine()) != null) {
                int index=s.lastIndexOf(" ");
                String sc=s.substring(index, s.length());

                System.out.println(sc);
                rt.exec("cmd /c Taskkill /PID" +sc+" /T /F");
                System.out.println("stop");

            }
        }catch(Exception e){
            e.printStackTrace();
        }
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

    @Test
    void t3() {
        System.out.println(Arrays.stream(
                        "com.sun.org.apache.xerces.internal.impl.dv.dtd.DTDDVFactoryImpl,com.sun.org.apache.xerces.internal.util.XMLChar,com.sun.org.apache.xerces.internal.impl.XMLEntityManager$EncodingInfo,com.sun.org.apache.xerces.internal.util.PropertyState,com.sun.org.apache.xerces.internal.impl.XMLScanner,com.sun.org.apache.xerces.internal.util.FeatureState,com.sun.org.apache.xerces.internal.xni.NamespaceContext,com.sun.org.apache.xerces.internal.impl.XMLDTDScannerImpl,jdk.xml.internal.JdkXmlUtils,com.sun.org.apache.xerces.internal.impl.dtd.XMLNSDTDValidator,com.sun.org.apache.xerces.internal.impl.XMLVersionDetector,com.sun.org.apache.xerces.internal.impl.XMLEntityManager,com.sun.org.apache.xerces.internal.impl.XMLDocumentFragmentScannerImpl,com.sun.org.apache.xerces.internal.impl.XMLNSDocumentScannerImpl,com.sun.org.apache.xerces.internal.impl.Constants,com.sun.org.apache.xerces.internal.util.XMLSymbols,com.sun.org.apache.xerces.internal.impl.dtd.XMLDTDValidator,com.sun.xml.internal.stream.util.ThreadLocalBufferAllocator,com.sun.org.apache.xerces.internal.impl.XMLEntityScanner,com.sun.org.apache.xerces.internal.impl.dtd.XMLDTDProcessor,com.sun.org.apache.xerces.internal.impl.XMLDocumentScannerImpl"
                                .split(","))
                .map(x -> "--initialize-at-build-time=" + x)
                .collect(Collectors.joining("\r\n")));
    }

}
