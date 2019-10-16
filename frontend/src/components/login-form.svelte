<script lang="ts">
	export let login: (username: string, password: string) => void;

	let username: string = '';
	const usernameChanged = (event: any) => username = event.target.value;

	let password: string = '';
	const passwordChanged = (event: any) => password = event.target.value;

	const submit = (event: any) => {
		event.preventDefault();
		if (typeof login === 'function') {
			login(username, password);
		}
	}
</script>

<style>
	form {
		padding: .5em 0;
	}
</style>

<form on:submit={submit}>
	<input on:keyup={usernameChanged} value={username}/>
	<input type="password" on:keyup={passwordChanged} value={password}/>
	<button>Login</button>
</form>
