package com.github.ik;


import org.apache.lucene.analysis.Analyzer;


public class IKAnalyzer extends Analyzer {

    private boolean useSmart = false;

    public IKAnalyzer() {
        this(false);
    }

    public IKAnalyzer(boolean useSmart) {
        super();
        this.useSmart = useSmart;
    }

    public boolean isUseSmart() {
        return useSmart;
    }

    public void setUseSmart(boolean useSmart) {
        this.useSmart = useSmart;
    }

    @Override
    protected TokenStreamComponents createComponents(String fieldName) {
        IKTokenizer tk = new IKTokenizer(this.useSmart);
        return new TokenStreamComponents(tk);
    }

}