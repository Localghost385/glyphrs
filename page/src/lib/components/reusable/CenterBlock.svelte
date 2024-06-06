<script>
	export let height;
	export let width;

	export let bg = "url('/src/lib/images/blob-scene-haikei.svg')";

	export let padding = 'p-5';
</script>

<div
	class="top-0 z-[-1] w-screen h-min-content p-20 min-h-[calc(100vh-80px)] bg-ctp-base flex flex-col items-center justify-center bg-[{bg}] bg-no-repeat bg-cover"
>
	<div
		class="relative w-[{width}] h-[{height}] border-[1px] border-ctp-text bg-ctp-base bg-opacity-60 backdrop-blur-xl flex flex-col items-center justify-between {padding} shadow-lg shadow-ctp-crust"
	>
		<slot />
	</div>
</div>
