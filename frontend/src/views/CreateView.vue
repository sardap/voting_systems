<script setup lang="ts">
import { useRoute } from 'vue-router';
import CreateElection from '@/components/CreateElection.vue';

const route = useRoute();
const api_key = route.query.api_key as string;

</script>

<template>
    <div>
        <h1>Create an election</h1>
        <p v-if="!api_key">Api key missing</p>
        <CreateElection v-else :api_key="api_key" />
    </div>
</template>

<style></style>
