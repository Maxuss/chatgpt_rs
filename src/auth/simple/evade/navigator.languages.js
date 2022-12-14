((languages) => {
    const langs = languages.length
        ? languages
        : ["en-US", "en"];
    utils.replaceGetterWithProxy(
        Object.getPrototypeOf(navigator),
        "languages",
        utils.makeHandler().getterValue(Object.freeze([...langs])),
    );
})