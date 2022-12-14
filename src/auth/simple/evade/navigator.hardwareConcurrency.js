
((hardwareConcurrency) => {
    utils.replaceGetterWithProxy(
        Object.getPrototypeOf(navigator),
        "hardwareConcurrency",
        utils.makeHandler().getterValue(hardwareConcurrency),
    );
});