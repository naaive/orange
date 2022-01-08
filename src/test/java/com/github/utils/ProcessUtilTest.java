package com.github.utils;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class ProcessUtilTest {

    @Test
    void isAlive() {
        boolean alive = ProcessUtil.isAlive();
        System.out.println(alive);
    }

    @Test
    void clean() {
    }
}