package com.github.executor;

import com.github.utils.ProcessUtil;
import lombok.extern.java.Log;

import java.util.logging.Level;

@Log
public class AliveExecutor implements Runnable {
    @Override
    public void run() {
        boolean debug = Boolean.parseBoolean(System.getProperty("debug"));
        if (debug) {
            return;
        }
        if (ProcessUtil.isAlive()) {
        } else {
            log.log(Level.SEVERE,"ui process dead, exit...");
            ProcessUtil.clean();
            Runtime.getRuntime().exit(1);
        }
    }
}
