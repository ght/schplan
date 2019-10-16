<script lang="ts">
	import LoginForm from './components/login-form.svelte';

	console.log('hi');
	const who = 'world';

	let count = 0;
	const increment = () => count += 1;

const login = (username: string, password: string) =>
	console.log(`Logging in "${username}:${password}".`);
</script>

<style>
	:global(body) {
		background-color: #333;
		color: #ccc;
		font-family: monospace;
	}
</style>

<!--<h1>Schplan</h1>-->
<button on:click={increment}>{count}</button>
<LoginForm {login}/>
