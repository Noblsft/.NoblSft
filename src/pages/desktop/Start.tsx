import {VStack, Button, Image } from "@chakra-ui/react";
import logo from "../../assets/logo.png";

export default function Start() {
    return (
        <VStack>
            <Image src={logo} alt="logo" aspectRatio={4 / 3} width="350px" />
            <Button colorScheme="teal" size="lg" width="30%">
                Create new nobl file
            </Button>
            <Button colorScheme="blue" size="lg" width="30%">
                Load existing nobl file
            </Button>
        </VStack>
    );
}