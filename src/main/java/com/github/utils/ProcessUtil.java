package com.github.utils;

import com.github.conf.IndexConf;
import lombok.extern.java.Log;

import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.util.Date;
import java.util.List;
import java.util.logging.Level;
import java.util.stream.Collectors;

import static com.github.conf.IndexConf.readFromFile;

@Log
public class ProcessUtil {

    private static final String ORANGE_CORE = "orange_core";

    public static boolean shouldStat() {

        IndexConf indexConf = readFromFile();
        Date lastStatTime = indexConf.getLastStatTime();
        Date now = new Date();
        indexConf.setLastStatTime(now);
        indexConf.save2file();

        return now.getTime()-lastStatTime.getTime()<1000*60*60*12;
    }
    public static void winKillByPort(int port) {
        try{
            Runtime rt = Runtime.getRuntime();
            Process proc = rt.exec("cmd /c netstat -ano | findstr " + port);

            BufferedReader stdInput = new BufferedReader(new
                    InputStreamReader(proc.getInputStream()));
            String s;
            if ((s = stdInput.readLine()) != null) {
                int index=s.lastIndexOf(" ");
                String sc=s.substring(index);
                rt.exec("cmd /c Taskkill /PID" +sc+" /T /F");
            }
        }catch(Exception e){
            log.log(Level.SEVERE, "kill err", e);
        }
    }

    public static boolean isAlive() {
        int cnt = 0;
        List<ProcessHandle> handles = ProcessHandle.allProcesses().collect(Collectors.toList());
        for (ProcessHandle handle : handles) {
            ProcessHandle.Info info = handle.info();
            if (info.command().isPresent()) {
                String s = info.command().get();
                if (s.contains(ORANGE_CORE)) {
                    cnt++;
                }
            }
            if (info.commandLine().isPresent()) {
                String s = info.commandLine().get();
                if (s.contains(ORANGE_CORE)) {
                    cnt++;

                }
            }
        }
        return cnt > 1;
    }

    public static void clean() {

        ProcessHandle.allProcesses().forEach(processHandle -> {
            ProcessHandle.Info info = processHandle.info();
            if (info.command().isPresent()) {
                String s = info.command().get();
                if (s.contains(IndexConf.FSEVENT_PATH)) {
                    processHandle.destroyForcibly();
                }
            }
            if (info.commandLine().isPresent()) {
                String s = info.commandLine().get();
                if (s.contains(IndexConf.FSEVENT_PATH)) {
                    processHandle.destroyForcibly();
                }
            }
        });
    }
}
