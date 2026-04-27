# ImmutableMe
Un contrato inteligente en Solana para identidades on-chain descentralizadas o en pocas palabras un LinkedIn descentralizado
Durante el desarrollo me topé con un muro interesante: la pestaña de Test automática del Solana Playground simplemente no quería cooperar. Por más que intentaba ejecutar las funciones desde la interfaz visual, no obtenía los resultados esperados.

Lejos de rendirme, descubrí que la verdadera forma de probar un contrato es dejando de lado los botones automáticos y escribiendo un cliente real en TypeScript. En cuanto ejecuté mi propio script, el contrato respondió perfectamente. Esto me enseñó que en Web3 no puedes confiar ciegamente en las herramientas visuales, la prueba de fuego siempre será el código hablando con el código.

¿Cómo funciona ImmutableMe?
A ver, te cuento cómo está la onda con mi programa. La idea es simple pero potente: quería crear un lugar donde pudieras guardar quién eres en la blockchain sin depender de que una empresa te dé permiso.

Lo primero que tienes que saber es que el programa usa algo llamado PDAs (Program Derived Addresses). Básicamente, son como "casilleros" que el programa crea. Para que no se mezclen los perfiles de todo el mundo, cada casillero tiene una combinación única: tu dirección de wallet y tu nickname. Así me aseguro de que tú seas el único que pueda abrir ese casillero.

Cuando le das al botón de Create, el programa aparta un pedacito de espacio en Solana. No le metí cosas de más para no gastar SOL innecesario; solo guardamos tu nickname, una bio cortita y tu link. Lo padre es que, al ser tu cuenta, tú pagas la "renta" por ese espacio, pero el programa está hecho para que siempre seas tú el dueño de esos datos.

Luego está la parte del Update. Aquí el código se pone estricto: antes de cambiar una sola letra de tu bio, revisa que la persona que firma la transacción sea la misma que creó el perfil. Si alguien más intenta meterle mano a tu cuenta, el programa le dice "naranjas" y cancela todo.

Y lo mejor para el final: el Delete. En la mayoría de las apps, cuando borras tu cuenta, tus datos se van, pero ellos se quedan con todo. Aquí no. Cuando decides cerrar tu perfil en ImmutableMe, el programa limpia el espacio y te devuelve los SOL que habías dejado como depósito de renta. Es como si el programa te diera las gracias por participar y te regresara tu dinero.

Al final del día, es un ciclo completo: creas, lees, cambias y borras, todo bajo tu control y sin intermediarios. ¡Pura magia de Solana!

Si quieres ver el contrato en acción, usa este script en tu cliente. Aquí te dejo cómo ejecutar la creación, lectura, actualización y borrado lo puedes hacer en un solo flujo o probarlo uno por uno:
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SolanaPortfolio;

  // Nickname unico para tu prueba
  const nickname = "User" + Math.floor(Math.random() * 1000); 
  
  const [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("profile"), provider.wallet.publicKey.toBuffer(), Buffer.from(nickname)],
    program.programId
  );

  console.log("Iniciando pruebas para el perfil:", nickname);

  // 1. CREATE: Registramos el perfil en la red
  await program.methods.createProfile(nickname, "Hola, soy desarrollador", "https://misitio.com").accounts({
    user: provider.wallet.publicKey,
    profile: profilePda,
    systemProgram: anchor.web3.SystemProgram.programId,
  }).rpc();
  console.log("Paso 1: Perfil creado exitosamente.");

  // 2. READ: Verificamos que los datos estan ahi
  const data = await program.account.userProfile.fetch(profilePda);
  console.log("Paso 2: Datos leidos -> Bio:", data.bio);

  // 3. UPDATE: Modificamos la informacion
  await program.methods.updateProfile(nickname, "Bio actualizada", "https://github.com").accounts({
    user: provider.wallet.publicKey,
    profile: profilePda,
  }).rpc();
  console.log("Paso 3: Perfil actualizado correctamente.");

  // 4. DELETE: Cerramos la cuenta y recuperamos el SOL de la renta
  await program.methods.deleteProfile(nickname).accounts({
    user: provider.wallet.publicKey,
    profile: profilePda,
  }).rpc();
  console.log("Paso 4: Perfil borrado y SOL de renta recuperado.");
}

main();
