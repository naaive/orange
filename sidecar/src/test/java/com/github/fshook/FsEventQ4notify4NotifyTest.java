package com.github.fshook;

import org.junit.jupiter.api.Test;

import java.util.List;

class FsEventQ4notify4NotifyTest {

    @Test
    void poll() {
        FsEventQ4notify q = new FsEventQ4notify("/");
        while (true) {
            List<FsLog> poll = q.poll(1);
            System.out.println(poll);
        }
    }
}