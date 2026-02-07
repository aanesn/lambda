<script>
	import Button from "$lib/components/Button.svelte"
</script>

<svelte:head>
	<title>Lambda</title>
	<meta name="description" content="A Lambda deployment tool" />
</svelte:head>

<div class="py-20 lg:py-40">
	<div class="flex flex-col items-center gap-y-6">
		<h1
			class="text-center font-manrope text-4xl font-semibold md:text-5xl lg:text-6xl xl:text-7xl"
		>
			Lambda is finally <br /> effortless
		</h1>
		<h2 class="max-w-lg text-center text-balance text-neutral-500 lg:text-lg">
			Deploy Lambda functions on your own AWS account without wrestling the console.
		</h2>
		<div class="flex items-center gap-x-6">
			<Button class="duration-300" href="/login">Start now</Button>
			<Button intent="secondary" class="duration-300">Docs</Button>
		</div>
	</div>
</div>
