package com.github.ik;


import org.apache.lucene.analysis.Tokenizer;
import org.apache.lucene.analysis.tokenattributes.CharTermAttribute;
import org.apache.lucene.analysis.tokenattributes.OffsetAttribute;
import org.apache.lucene.analysis.tokenattributes.TypeAttribute;
import org.wltea.analyzer.core.IKSegmenter;
import org.wltea.analyzer.core.Lexeme;

import java.io.IOException;


public class IKTokenizer extends Tokenizer {

    private final IKSegmenter ikSegmenter;

    private final CharTermAttribute termAtt;
    private final OffsetAttribute offsetAtt;
    private final TypeAttribute typeAtt;
    private int endPosition;

    public IKTokenizer(boolean useSmart) {
        super();
        offsetAtt = addAttribute(OffsetAttribute.class);
        termAtt = addAttribute(CharTermAttribute.class);
        typeAtt = addAttribute(TypeAttribute.class);
        ikSegmenter = new IKSegmenter(input, useSmart);
    }


    @Override
    final public boolean incrementToken() throws IOException {
        clearAttributes();
        Lexeme nextLexeme = ikSegmenter.next();
        if (nextLexeme != null) {
            termAtt.append(nextLexeme.getLexemeText());
            termAtt.setLength(nextLexeme.getLength());
            offsetAtt.setOffset(nextLexeme.getBeginPosition(),
                    nextLexeme.getEndPosition());
            endPosition = nextLexeme.getEndPosition();
            typeAtt.setType(nextLexeme.getLexemeTypeString());
            return true;
        }
        return false;
    }


    @Override
    public void reset() throws IOException {
        super.reset();
        ikSegmenter.reset(input);
    }

    @Override
    public final void end() {
        int finalOffset = correctOffset(this.endPosition);
        offsetAtt.setOffset(finalOffset, finalOffset);
    }
}