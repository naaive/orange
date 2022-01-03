package com.github.fshook;

import java.io.File;
import java.util.Arrays;
import java.util.List;

public class FsEventQTest {

    public static void main(String[] args) throws InterruptedException {

        FsEventQ q = new FsEventQ(Arrays.stream(File.listRoots()).map(File::getAbsolutePath)
                .toArray(String[]::new));

        while (true) {
            List<FsLog> poll = q.poll(4);
            System.out.println(poll);
        }
    }
}
