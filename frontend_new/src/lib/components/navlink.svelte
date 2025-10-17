<script lang="ts">
  let { href, title, ...props } = $props();
</script>

<a {href} {...props}>{title}</a>
