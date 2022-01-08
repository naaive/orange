package com.github.utils;

import com.fasterxml.jackson.databind.ObjectMapper;
import lombok.SneakyThrows;

public class JsonUtil {

    private static final ObjectMapper gson = new ObjectMapper();

    @SneakyThrows
    public static <T> T fromJson(String json, Class<T> typeOfT)  {
        return gson.readValue(json, typeOfT);
    }

    @SneakyThrows
    public static String toJson(Object src) {
        return gson.writeValueAsString(src);
    }
}
