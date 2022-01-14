package org.apache.lucene.util;

import org.apache.lucene.analysis.tokenattributes.PackedTokenAttributeImpl;

import java.lang.reflect.Constructor;
import java.lang.reflect.UndeclaredThrowableException;

public final class AttributeFactoryHelpers {
    static final AttributeFactory DEFAULT = new DefaultAttributeFactory();

    static final Constructor<? extends AttributeImpl> findAttributeImplCtor(Class<? extends AttributeImpl> clazz) {
        try {
            return clazz.getConstructor();
        } catch (NoSuchMethodException e) {
            throw new IllegalArgumentException("Cannot lookup accessible no-arg constructor for: " + clazz.getName(), e);
        }
    }

    static final class DefaultAttributeFactory extends AttributeFactory {
        private final ClassValue<Constructor<? extends AttributeImpl>> constructors = new ClassValue<Constructor<? extends AttributeImpl>>() {
            @Override
            protected Constructor<? extends AttributeImpl> computeValue(Class<?> attClass) {
                return AttributeFactoryHelpers.findAttributeImplCtor(findImplClass(attClass.asSubclass(Attribute.class)));
            }
        };

        DefaultAttributeFactory() {}

        @Override
        public AttributeImpl createAttributeInstance(Class<? extends Attribute> attClass) {
            try {
                return constructors.get(attClass).newInstance();
            } catch (Error | RuntimeException e) {
                throw e;
            } catch (Throwable e) {
                throw new UndeclaredThrowableException(e);
            }
        }

        private Class<? extends AttributeImpl> findImplClass(Class<? extends Attribute> attClass) {
            try {
                return Class.forName(attClass.getName() + "Impl", true, attClass.getClassLoader()).asSubclass(AttributeImpl.class);
            } catch (ClassNotFoundException cnfe) {
                throw new IllegalArgumentException("Cannot find implementing class for: " + attClass.getName());
            }
        }
    }

    public static <A extends AttributeImpl> AttributeFactory getStaticImplementation(AttributeFactory delegate, Class<A> clazz) {
//        final Constructor<? extends AttributeImpl> constr = AttributeFactoryHelpers.findAttributeImplCtor(clazz);
        return new AttributeFactory.StaticImplementationAttributeFactory<A>(delegate, clazz) {
            @Override
            protected A createInstance() {
                try {

                    return (A) new PackedTokenAttributeImpl();
                } catch (Error | RuntimeException e) {
                    throw e;
                } catch (Throwable e) {
                    throw new UndeclaredThrowableException(e);
                }
            }
        };
    }
}