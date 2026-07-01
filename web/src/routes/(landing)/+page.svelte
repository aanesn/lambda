<script lang="ts">
	import Button from "$lib/components/Button.svelte"
	import Link from "$lib/components/Link.svelte"
</script>

<svelte:head>
	<title>lambda</title>
	<meta name="description" content="a lambda deployment tool" />
</svelte:head>

<div class="py-40">
	<div class="flex flex-col items-center justify-center gap-y-6">
		<h1
			class="text-center text-4xl md:text-5xl lg:text-6xl font-manrope font-semibold tracking-tight"
		>
			aws lambda<br /> without the aws.
		</h1>
		<h2 class="text-center text-neutral-400 text-balance max-w-sm lg:text-lg">
			deploy lambda functions without wrestling with the console.
		</h2>
		<div class="flex items-center gap-x-6">
			<Button class="duration-300">start now</Button>
			<Link class="duration-300" href="https://github.com/aanesn/lambda">repo</Link>
		</div>
	</div>
</div>
