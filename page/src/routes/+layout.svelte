<script>
	import Nav from '$lib/components/universal/nav.svelte';
	import Footer from '$lib/components/universal/footer.svelte';
	import { Spinner } from 'flowbite-svelte';

	import { beforeNavigate, afterNavigate } from '$app/navigation';

	let opacity = '0';
	beforeNavigate(() => {
		opacity = '100';
	});
	afterNavigate(() => {
		opacity = '0';
	});

	import '$lib/app.css';
</script>

<div class=" dark:bg-gray-900 transition-colors duration-300">
	<Nav />

	<div class="hidden visible" />

	<slot />

	<Footer />

	{#if opacity != '0'}
		<div class="fixed top-0 left-0 right-0 bottom-0 flex items-center justify-center">
			<div class="text-center transition-all duration-300 opacity-{opacity} ease-in-out">
				<Spinner size="20" />
			</div>
		</div>
	{/if}
</div>

<style lang="postcss">
	@import url('https://fonts.googleapis.com/css2?family=Urbanist:ital,wght@0,100..900;1,100..900&display=swap');
	@import url('https://fonts.googleapis.com/css2?family=Fira+Code&display=swap');
	* {
		font-family: 'Urbanist', sans-serif;
		font-optical-sizing: auto;
		font-weight: 300;
		font-style: normal;
	}

	.fira-code {
		font-family: 'Fira Code', monospace;
		font-optical-sizing: auto;
		font-weight: 400;
		font-style: normal;
	}
</style>
