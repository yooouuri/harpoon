<script setup lang="ts">
import { computed, reactive, ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/tauri";
import { Store } from "tauri-plugin-store-api";
import { randomUUID } from "uncrypto";
import HInput from "@/components/HInput.vue";
import HCheckbox from "@/components/HCheckbox.vue";
import HButton from "@/components/HButton.vue";

const router = useRouter();

async function back() {
    await router.back();
}

const route = useRoute();

const backButton = computed(() => route.meta.backButton);

const connected = ref(false);

const store = new Store(".connections.dat");

type ConnectionType = "socket" | "http";

type Connection = {
    name: string;
    path: string;
    connectionType: ConnectionType;
    saveConnection: boolean;
};

const connections = await store.entries<Connection>();

async function connect() {
    try {
        await invoke("connect", { path: connection.path });

        if (connection.saveConnection) {
            await store.set(randomUUID(), {
                path: connection.path,
                connectionType: connection.connectionType,
                name: connection.name,
            });
        }

        // hide form
        //
        connected.value = true;
    } catch (e) {
        console.error(e);
    }
}

const connection = reactive<Connection>({
    name: "Colima",
    path: "unix:///Users/youri/.colima/docker.sock",
    connectionType: "socket",
    saveConnection: false,
});

async function openConnection({ path, connectionType }: Connection) {
    // TODO use connectionType

    await invoke("connect", { path });

    connected.value = true;
}
</script>

<template>
    <div class="m-10" v-if="!connected">
        <div>LOGO</div>
        <h1>Saved connections</h1>

        <div class="grid grid-cols-3 gap-4">
            <div
                class="border m-4 rounded p-6 flex flex-col relative"
                v-for="connection in connections"
                :key="connection[0]"
            >
                <div>{{ connection[1].name }}</div>
                <div>{{ connection[1].connectionType }}</div>

                <div class="mt-5 flex gap-2 absolute right-6 top-0">
                    <div
                        class="icon-external-link cursor-pointer"
                        @click="openConnection(connection[1])"
                    ></div>
                    <div class="icon-pencil cursor-pointer"></div>
                    <div class="icon-trash-2 cursor-pointer"></div>
                </div>
            </div>
        </div>

        {{ connection }}

        <h1>Add new connection</h1>

        <div class="w-96">
            <HInput label="Name" v-model="connection.name" />
            <HInput
                label="Connection type"
                v-model="connection.connectionType"
            />
            <HInput
                label="Path"
                v-model="connection.path"
                v-if="connection.connectionType === 'socket'"
            />

            <HCheckbox
                label="Save connection"
                v-model="connection.saveConnection"
            />

            <HButton @click="connect">Connect</HButton>

            Cancel
        </div>
    </div>
    <div class="flex" v-else>
        <aside class="w-64">
            <ul class="m-4">
                <li class="text-lg pb-4">
                    <RouterLink to="/" class="flex items-center gap-2">
                        <span class="icon-container"></span>
                        Containers
                    </RouterLink>
                </li>
                <li class="text-lg pb-4">
                    <RouterLink to="/" class="flex items-center gap-2">
                        <span class="icon-box"></span>
                        Images
                    </RouterLink>
                </li>
            </ul>
        </aside>
        <div class="w-full relative">
            <div class="h-16 flex items-center" v-if="backButton">
                <div
                    class="z-40 text-3xl icon-chevron-left cursor-pointer"
                    @click="back"
                ></div>
            </div>
            <RouterView v-slot="{ Component }">
                <suspense timeout="0">
                    <div>
                        <component
                            :is="Component"
                            :key="$route.path"
                        ></component>
                    </div>
                </suspense>
            </RouterView>
        </div>
    </div>
</template>
