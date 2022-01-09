package com.github.executor;

import com.github.utils.ProcessUtil;

public class AliveExecutor implements Runnable {
    @Override
    public void run() {
        if (ProcessUtil.isAlive()) {
        } else {
            ProcessUtil.clean();
            Runtime.getRuntime().exit(1);
        }
    }
}
