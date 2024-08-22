<script setup lang="ts">
import { get_token } from '@/backend'
import { onMounted, ref } from 'vue'

const props = defineProps({
  election_id: {
    type: String,
    required: true
  },
  api_key: {
    type: String,
    required: true
  },
  election_path: {
    type: String,
    required: true
  }
})

let loading = ref<boolean>(true)
let link = ref<string>('')

async function update_token() {
  loading.value = true
  console.log('Getting token')
  const last_token = await get_token(props.api_key, props.election_id)
  link.value = `${window.location.origin}/${props.election_path}/${props.election_id}?vote_token=${last_token}`
  console.log(`Got token`)
  loading.value = false
}

onMounted(async () => {
  await update_token()
})

async function copy_link_to_clipboard() {
  navigator.clipboard.writeText(link.value)
  await update_token()
}
</script>

<template>
  <div>
    <div v-if="loading">
      <p>Loading...</p>
    </div>
    <div v-else>
      <p>
        <a :href="link">{{ link }}</a>
      </p>
      <button @click="copy_link_to_clipboard">Copy and get next</button>
    </div>
  </div>
</template>

<style scoped></style>
