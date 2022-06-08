import { AuthClient } from "@dfinity/auth-client";
import { Actor, Identity, HttpAgent } from "@dfinity/agent";
// import { canisterId, createActor } from "../../../declarations/signer";
import { idlFactory } from "./signer.did.js";

const backend = process.env.IC_SIGNER_CANISTER_ID;

export async function handleAuthenticated(authClient: AuthClient) {
    const identity = (await authClient.getIdentity()) as unknown as Identity;
    const actor = getBackendActor(identity);

    // Invalidate identity then render login when user goes idle
    authClient.idleManager?.registerCallback(() => {
        Actor.agentOf(actor)?.invalidateIdentity?.();
        // renderIndex();
    });
}

export function getBackendActor(identity?: Identity) {
    return createBackendActor(backend as string, {
        agentOptions: identity ? {
            identity,
        } : undefined,
    });
}

function createBackendActor(canisterId, options) {
    const agent = new HttpAgent({ ...options?.agentOptions });

    // Fetch root key for certificate validation during development
    if (process.env.NODE_ENV !== "production") {
        agent.fetchRootKey().catch(err => {
            console.warn("Unable to fetch root key. Check to ensure that your local replica is running");
            console.error(err);
        });
    }

    // Creates an actor with using the candid interface and the HttpAgent
    return Actor.createActor(idlFactory, {
        agent,
        canisterId,
        ...options?.actorOptions,
    });
};