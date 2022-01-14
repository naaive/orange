package com.github.accessor;

import lombok.Data;

import java.time.LocalDateTime;

@Data
public class StatProcess {
    private LocalDateTime lastStart;
    private LocalDateTime lastEnd;
    private StatProcessStatus status;
}
