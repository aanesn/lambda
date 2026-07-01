<script lang="ts">
	import Link from "$lib/components/Link.svelte"
</script>

<footer class="flex items-center justify-between text-sm py-6">
	<span class="text-neutral-500">© {new Date().getFullYear()} lambda</span>
	<Link href="/" intent="secondary">privacy and terms</Link>
</footer>
