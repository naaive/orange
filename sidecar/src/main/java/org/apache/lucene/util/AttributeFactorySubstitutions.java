//package org.apache.lucene.util;
//
//import com.oracle.svm.core.annotate.*;
//
//import java.lang.invoke.MethodHandle;
//import java.lang.reflect.Constructor;
//import java.lang.reflect.UndeclaredThrowableException;
//
//@TargetClass(AttributeFactory.class)
//final class AttributeFactorySubstitutions {
//    @Alias
//    @RecomputeFieldValue(kind = RecomputeFieldValue.Kind.FromAlias, isFinal = true)
//    public static AttributeFactory DEFAULT_ATTRIBUTE_FACTORY = new AttributeFactory() {
//        private final ClassValue<Constructor<? extends AttributeImpl>> constructors = new ClassValue<Constructor<? extends AttributeImpl>>() {
//            @Override
//            protected Constructor<? extends AttributeImpl> computeValue(Class<?> attClass) {
//                return AttributeFactoryHelpers.findAttributeImplCtor(findImplClass(attClass.asSubclass(Attribute.class)));
//            }
//        };
//
//        @Override
//        public AttributeImpl createAttributeInstance(Class<? extends Attribute> attClass) {
//            try {
//                return constructors.get(attClass).newInstance();
//            } catch (Error | RuntimeException e) {
//                throw e;
//            } catch (Throwable e) {
//                throw new UndeclaredThrowableException(e);
//            }
//        }
//
//        private Class<? extends AttributeImpl> findImplClass(Class<? extends Attribute> attClass) {
//            try {
//                return Class.forName(attClass.getName() + "Impl", true, attClass.getClassLoader()).asSubclass(AttributeImpl.class);
//            } catch (ClassNotFoundException cnfe) {
//                throw new IllegalArgumentException("Cannot find implementing class for: " + attClass.getName());
//            }
//        }
//    };
//
//    @Substitute
//    public static <A extends AttributeImpl> AttributeFactory getStaticImplementation(AttributeFactory delegate, Class<A> clazz) {
//        final Constructor<? extends AttributeImpl> constr = AttributeFactoryHelpers.findAttributeImplCtor(clazz);
//        return new AttributeFactory.StaticImplementationAttributeFactory<A>(delegate, clazz) {
//            @Override
//            protected A createInstance() {
//                try {
//                    return (A) constr.newInstance();
//                } catch (Error | RuntimeException e) {
//                    throw e;
//                } catch (Throwable e) {
//                    throw new UndeclaredThrowableException(e);
//                }
//            }
//        };
//    }
//
//    @Delete
//    static final MethodHandle findAttributeImplCtor(Class<? extends AttributeImpl> clazz) {
//        throw new RuntimeException();
//    }
//}