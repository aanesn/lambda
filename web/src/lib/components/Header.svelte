<script lang="ts">
	import Logomark from "$lib/assets/logomark.svg"
	import Button from "./Button.svelte"
	import Link from "./Link.svelte"
</script>

<header class="relative flex h-[72px] items-center justify-between">
	<a href="/">
		<img src={Logomark} alt="logomark" />
	</a>
	<nav class="absolute left-1/2 hidden -translate-x-1/2 items-center gap-x-6 lg:flex">
		<Link href="/">Products</Link>
		<Link href="/">Solutions</Link>
		<Link href="/">Customers</Link>
		<Link href="/">Resources</Link>
		<Link href="/">Pricing</Link>
	</nav>
	<div class="hidden items-center gap-x-4 lg:flex">
		<Button class="duration-300">Start now</Button>
		<Link href="/" class="duration-300">Log in</Link>
	</div>
</header>
