package com.github;

import org.junit.jupiter.api.Test;

import java.awt.*;
import java.io.File;
import java.io.IOException;

public class FolderTest {

    @Test
    public void t1() throws IOException {
        Desktop.getDesktop().open(new File("C:\\Drivers\\Lan.Intel"));
    }
}

