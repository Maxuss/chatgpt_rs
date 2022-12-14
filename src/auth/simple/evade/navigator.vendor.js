((vendor) => {
    utils.replaceGetterWithProxy(
        Object.getPrototypeOf(navigator),
        "vendor",
        utils.makeHandler().getterValue(vendor),
    );
})