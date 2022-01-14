package com.github;

import org.junit.jupiter.api.Test;

import java.io.File;
import java.io.IOException;

public class FolderTest {

    @Test
    public void t1() throws IOException {
        File x = new File("");
        System.out.println(x.getAbsolutePath());
        System.out.println(x);
    }
}

