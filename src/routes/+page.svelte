<script lang="ts">
   import { onMount } from 'svelte';
   import init, { handle_data } from '$lib/valut39_wasm_pack/pkg';

   let wasmModule;
   let inputSeed = '';
   let password = '';
   let cipher = '';
   let passwordWarning = '';
   let showPassword = false;
   let seedWarning = '';
   let seedWordCount = 0;
   let minimumWordCount = 12;

   onMount(async () => {
   	const { default: init } = await import('$lib/valut39_wasm_pack/pkg');
   	wasmModule = await init();
   });

   $: seedWordCount = inputSeed.split(' ').filter((word) => word.length > 0).length;

   function togglePasswordVisibility() {
   	showPassword = !showPassword;
   }

   async function handleFormSubmit() {
   	seedWarning = '';
   	cipher = '';
   	const inputWords = inputSeed.trim().split(/\s+/);
   	seedWordCount = inputWords.length;

   	if (seedWordCount < 12 || seedWordCount > 24) {
   		seedWarning = 'Please enter a seed phrase between 12 and 24 words.';
   		return;
   	}

   	try {
   		const uniqueWordsSet = new Set(inputWords);

   		if (uniqueWordsSet.size !== inputWords.length) {
   			seedWarning = 'Do not enter the same word twice in your seed phrase.';
   			return;
   		}
   	} catch (error) {
   		console.error('Error:', error);
   	}

   	try {
   		cipher = await handle_data(inputSeed, password);
   	} catch (error) {
   		console.error('Error invoking handle_data:', error);
   	}
   }

   function copyToClipboard(text: string) {
   	navigator.clipboard.writeText(text).then(
   		function () {
   			console.log('Copy successful');
   		},
   		function (err) {
   			console.error('Copy failed', err);
   		}
   	);
   }

   const handlePaste = (event: ClipboardEvent) => {
   	event.preventDefault();
   	const clipboardData = event.clipboardData;
   	if (clipboardData) {
   		const pastedData = clipboardData.getData('text');
   		inputSeed = pastedData.replace(/\r\n/g, '\n').replace(/\n/g, ' ');
   	}
   };
</script>

<main>
	<h1>Valut39</h1>
	<h2>Generate encryption</h2>
	<p>Please enter your wallet's seed phrase and your chosen password here.</p>
	<p>
		Never forget your password, as it is required to restore your seed phrase from the encrypted
		text.
	</p>
	<p>Please enter your password using only alphanumeric characters, without spaces.</p>
	<p>
		We recommend using this application in an offline environment to reduce the risk of hacking.
	</p>

	<form on:submit|preventDefault={handleFormSubmit}>
		<label>
			<p>Seed phrase:</p>
			<textarea bind:value={inputSeed} rows="2" on:paste={handlePaste}></textarea>
		</label>
		<p>Number of words entered: {seedWordCount}</p>
		{#if seedWarning}
			<p class="warning">{seedWarning}</p>
		{/if}

		<label>
			Password:
			{#if showPassword}
				<input
					type="text"
					bind:value={password}
					minlength="4"
					maxlength="80"
					class="password-input"
				/>
			{:else}
				<input type="password" bind:value={password} minlength="8" class="password-input" />
			{/if}
			<button type="button" on:click={() => copyToClipboard(password)}>Copy password</button>
			<button class="enabled" type="button" on:click={togglePasswordVisibility}
				>{showPassword ? 'Hide' : 'Show'}</button
			>
		</label>
		{#if passwordWarning}
			<p class="warning">{passwordWarning}</p>
		{/if}

		<button class="generate" type="submit">Generate encrypted text</button>
	</form>

	{#if cipher}
		<p>Encrypted text: {cipher}</p>
		<button type="button" on:click={() => copyToClipboard(cipher)}>Copy encrypted text</button>
	{/if}
</main>

<style>
	:global(body) {
		margin: 0;
		background: linear-gradient(to bottom right, #afeeee 33%, #f0e68c 66%);
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 100vh;
	}

	main {
		padding: 20px;
		font-family: sans-serif;
		color: #696969;
		width: 66%;
		box-sizing: border-box;
	}

	h1 {
		font-size: 24px;
		margin-bottom: 20px;
	}

	h2 {
		font-size: 20px;
		margin-bottom: 15px;
	}

	form {
		margin: 6px 0;
	}

	p {
		font-size: 16px;
		margin-bottom: 10px;
	}

	label {
		display: block;
		margin-bottom: 5px;
	}

	textarea {
		padding: 5px;
		font-size: 16px;
		width: 48vw;
		height: 12vw;
		margin-bottom: 5px;
	}

	input {
		margin-top: 25px;
		padding: 5px;
		font-size: 16px;
		width: 40vw;
		margin-bottom: 10px;
	}

	button {
		padding: 5px 10px;
		font-size: 16px;
		cursor: pointer;
	}
	.enabled {
		margin-top: 15px;
	}
	.generate {
		margin-top: 15px;
	}
</style>
