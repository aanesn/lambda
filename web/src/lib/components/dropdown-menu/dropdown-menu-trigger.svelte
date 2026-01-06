<script lang="ts">
	import { cx } from "$lib/utils"
	import { DropdownMenu } from "bits-ui"

	let { class: className, ...restProps }: DropdownMenu.TriggerProps = $props()
</script>

<DropdownMenu.Trigger class={cx("focus:outline-none", className)} {...restProps} />
