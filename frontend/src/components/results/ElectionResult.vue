<script setup lang="ts">
import { ref, type PropType, onMounted } from 'vue';
import MakeToken from '@/components/MakeToken.vue';
import { change_extra, election_type_to_path, get_extra, type ElectionBase, type ElectionType, type GetExtraResponse } from '@/backend';

const props = defineProps({
    api_key: {
        type: String,
        required: true,
    },
    election_type: {
        type: Object as PropType<ElectionType>,
        required: true,
    },
    election: {
        type: Object as PropType<ElectionBase>,
        required: true,
    }
});

const link = ref<string>(window.location.href);
const vote_link = ref<string>(`${window.location.origin}/${election_type_to_path(props.election_type)}/${props.election.id}`);
const extra = ref<GetExtraResponse | null>(null);
const loading = ref(false);

async function refresh() {
    loading.value = true;
    extra.value = await get_extra(props.election.id, props.api_key);
    loading.value = false;
}

onMounted(async () => {
    await refresh()
});

function copy_link_to_clipboard() {
    navigator.clipboard.writeText(window.location.href);
}

async function update_extra() {
    loading.value = true;
    await change_extra(props.election.id, props.api_key, extra.value?.locked, extra.value?.voting_lock);
    loading.value = false;
}

</script>

<template>
    <div>
        <h3>Title: {{ election.title }}</h3>
        <div>
            <p>Remember to save this link or you won't be able to get back here</p>
            <p><a :href="link">{{ link }}</a></p>
            <button @click="() => copy_link_to_clipboard()">Copy</button>
        </div>
        <hr />
        <div>
            <h2>Vote link</h2>
            <div v-if="election.require_token">
                <p>This is election requires token so you must send every voter a unique link</p>
                <MakeToken :election_path="election_type_to_path(props.election_type)" :election_id="election.id"
                    :api_key="api_key" />
            </div>
            <div v-else>
                <p>This link works for everyone</p>
                <p><a :href="vote_link">{{ vote_link }}</a></p>
            </div>
        </div>
        <hr />
        <br />
        <div>
            <div v-if="loading">
                <p>Loading...</p>
            </div>
            <div v-else>
                <p>Edit Election</p>
                <div v-if="extra">
                    <div>
                        <input v-model="extra.locked" type="checkbox" id="public" />
                        <label for="public"> Results Public</label>
                    </div>
                    <div>
                        <input v-model="extra.voting_lock" type="checkbox" id="locked_voting" />
                        <label for="locked_voting"> Closed for voting</label>
                    </div>
                    <button @click="update_extra">Update</button>
                </div>
            </div>
        </div>
        <hr />
        <br />
    </div>
</template>

<style scoped></style>
