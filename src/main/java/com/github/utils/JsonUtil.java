package com.github.utils;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import lombok.extern.java.Log;

import java.io.IOException;
import java.util.logging.Level;

@Log
public class JsonUtil {

    private static final ObjectMapper gson = new ObjectMapper();

    public static <T> T fromJson(String json, Class<T> typeOfT) {
        try {
            return gson.readValue(json, typeOfT);
        } catch (IOException e) {
            log.log(Level.SEVERE, "fromJson err", e);
            throw new RuntimeException("from json err");
        }
    }

    public static String toJson(Object src) {
        try {
            return gson.writeValueAsString(src);
        } catch (JsonProcessingException e) {
            log.log(Level.SEVERE, "open folder err", e);
            throw new RuntimeException("to json err");
        }
    }
}
