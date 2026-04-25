# ImmutableMe
Un contrato inteligente en Solana para identidades on-chain descentralizadas
Durante el desarrollo me topé con un muro interesante: la pestaña de Test automática del Solana Playground simplemente no quería cooperar. Por más que intentaba ejecutar las funciones desde la interfaz visual, no obtenía los resultados esperados.

Lejos de rendirme, descubrí que la verdadera forma de probar un contrato es dejando de lado los botones automáticos y escribiendo un cliente real en TypeScript. En cuanto ejecuté mi propio script, el contrato respondió perfectamente. Esto me enseñó que en Web3 no puedes confiar ciegamente en las herramientas visuales, la prueba de fuego siempre será el código hablando con el código.

Si quieres ver el contrato en acción, usa este script en tu cliente. Aquí te dejo cómo ejecutar la creación, lectura, actualización y borrado en un solo flujo:
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
