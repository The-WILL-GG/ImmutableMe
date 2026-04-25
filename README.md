# ImmutableMe
Un contrato inteligente en Solana para identidades on-chain descentralizadas
Durante el desarrollo me topé con un muro interesante: la pestaña de Test automática del Solana Playground simplemente no quería cooperar. Por más que intentaba ejecutar las funciones desde la interfaz visual, no obtenía los resultados esperados.

Lejos de rendirme, descubrí que la verdadera forma de probar un contrato es dejando de lado los botones automáticos y escribiendo un cliente real en TypeScript. En cuanto ejecuté mi propio script, el contrato respondió perfectamente. Esto me enseñó que en Web3 no puedes confiar ciegamente en las herramientas visuales; la prueba de fuego siempre será el código hablando con el código.
