package com.github.fshook;

import com.github.conf.AppConf;
import com.github.utils.ProcessUtil;
import lombok.extern.java.Log;

import java.io.BufferedReader;
import java.io.File;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Objects;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.ThreadPoolExecutor;
import java.util.concurrent.TimeUnit;
import java.util.logging.Level;

@Log
public class FsEventQ {

    private final ArrayBlockingQueue<FsLog> fsLogs = new ArrayBlockingQueue<>(1024);

    public FsEventQ(String... roots) {
        int len = roots.length;
        if (len == 0) {
            throw new IllegalArgumentException();
        }

        ProcessUtil.cleanFsevent();

        ThreadPoolExecutor poolExecutor =
                new ThreadPoolExecutor(len, len, 0L, TimeUnit.MILLISECONDS, new ArrayBlockingQueue<>(len));
        for (String root : roots) {
            poolExecutor.submit(() -> newListener(AppConf.FSEVENT_PATH + " " + root));
        }
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
            String absolutePath = new File(cmd).getAbsolutePath();
            p = Runtime.getRuntime().exec(absolutePath);
            BufferedReader br =
                    new BufferedReader(new InputStreamReader(p.getErrorStream(), StandardCharsets.UTF_8), 256);
            String line;
            while ((line = br.readLine()) != null) {
                log.log(Level.FINE, line);
                String[] split = line.split(" ");
                String op = split[0];
                String file = split[1];
                if (Objects.equals("Modified", op)) {
                    if (Objects.equals("file:", file)) {
                        fsLogs.put(new FsLog().setCmd(Cmd.U).setPath(line.substring("Modified file: ".length())));
                    }
                    if (Objects.equals("directory:", file)) {
                        fsLogs.put(new FsLog().setCmd(Cmd.U).setPath(line.substring("Modified directory: ".length())));
                    }
                }
                if (Objects.equals("Deleted", op)) {
                    if (Objects.equals("file:", file)) {
                        fsLogs.put(new FsLog().setCmd(Cmd.D).setPath(line.substring("Deleted file: ".length())));
                    }
                    if (Objects.equals("directory:", file)) {
                        fsLogs.put(new FsLog().setCmd(Cmd.U).setPath(line.substring("Deleted directory: ".length())));
                    }
                }
                if (Objects.equals("Created", op)) {
                    if (Objects.equals("file:", file)) {
                        fsLogs.put(new FsLog().setCmd(Cmd.C).setPath(line.substring("Created file: ".length())));
                    }
                    if (Objects.equals("directory:", file)) {
                        fsLogs.put(new FsLog().setCmd(Cmd.U).setPath(line.substring("Created directory: ".length())));
                    }
                }
            }
        } catch (IOException | InterruptedException e) {
            log.log(Level.SEVERE, "newListener err", e);
        } finally {
            if (p != null) {
                p.destroy();
            }
            Runtime.getRuntime().exit(-1);
        }
    }
}
