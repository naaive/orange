package com.github.fshook;

import com.github.conf.AppConf;
import com.github.utils.ProcessUtil;
import lombok.extern.java.Log;

import java.io.BufferedReader;
import java.io.File;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.util.*;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.ThreadPoolExecutor;
import java.util.concurrent.TimeUnit;
import java.util.logging.Level;

@Log
public class FsEventQ4notify {

    private final ArrayBlockingQueue<FsLog> fsLogs = new ArrayBlockingQueue<>(1024);
    public final ThreadPoolExecutor poolExecutor = new ThreadPoolExecutor(1, 1, 0L, TimeUnit.MILLISECONDS, new ArrayBlockingQueue<>(1));;

    public FsEventQ4notify(String... roots) {
        int len = roots.length;
        if (len == 0) {
            throw new IllegalArgumentException();
        }

        ProcessUtil.cleanFsevent();
        poolExecutor.submit(() -> newListener(AppConf.FSEVENT_PATH + " " + String.join(" ", roots)));
    }

    public List<FsLog> poll(int size) {
        try {
            List<FsLog> fsLogs0 = new ArrayList<>(size);
            for (int i = 0; i < size; i++) {
                FsLog take = fsLogs.poll(200, TimeUnit.MILLISECONDS);
                if (take != null) {
                    fsLogs0.add(take);
                }
            }
            return fsLogs0;
        } catch (InterruptedException e) {
            log.log(Level.SEVERE, "poll err", e);
        }
        return Collections.emptyList();
    }

    private void newListener(String cmd) {
        Process p = null;
        try {
            String[] s = cmd.split(" ");
            String abs = s[0];
            String arg = cmd.substring(abs.length()+1);
            String absolutePath = new File(abs).getAbsolutePath();
            String command = absolutePath + " " + arg;

            p = Runtime.getRuntime().exec(command);
            BufferedReader br =
                    new BufferedReader(new InputStreamReader(p.getErrorStream(), StandardCharsets.UTF_8), 256);
            String line;
            while ((line = br.readLine()) != null) {
                log.log(Level.FINE, line);
                String[] split = line.split(" ");
                String op = split[0];
                //todo multi op

                String file = split[split.length-1];
                if (Objects.equals("CHMOD", op)) {
                    fsLogs.put(new FsLog().setCmd(Cmd.U).setPath(file.substring(1, file.length() - 1)));
                }
                if (Objects.equals("CREATE", op)) {
                    fsLogs.put(new FsLog().setCmd(Cmd.C).setPath(file.substring(1, file.length() - 1)));
                }
                if (Objects.equals("REMOVE", op)) {
                    fsLogs.put(new FsLog().setCmd(Cmd.D).setPath(file.substring(1, file.length() - 1)));
                }
                if (Objects.equals("RENAME", op)) {
                    fsLogs.put(new FsLog().setCmd(Cmd.U).setPath(file.substring(1, file.length() - 1)));
                }
                if (Objects.equals("WRITE", op)) {
                    fsLogs.put(new FsLog().setCmd(Cmd.U).setPath(file.substring(1, file.length() - 1)));
                }
                if (Objects.equals("CLOSE_WRITE", op)) {
                    fsLogs.put(new FsLog().setCmd(Cmd.U).setPath(file.substring(1, file.length() - 1)));
                }
                if (Objects.equals("RESCAN", op)) {
                    fsLogs.put(new FsLog().setCmd(Cmd.U).setPath(file.substring(1, file.length() - 1)));
                }
            }
        } catch (IOException | InterruptedException e) {
            log.log(Level.SEVERE, "newListener err", e);
        }catch (Exception e){
            log.log(Level.SEVERE, "fsevent err", e);
        }finally {
            if (p != null) {
                p.destroy();
            }
            log.log(Level.SEVERE, "fsevent err");
            Runtime.getRuntime().exit(-1);
        }
    }
}
