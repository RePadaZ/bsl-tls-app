export interface Root {
    issues: {
        primaryLocation: {
            message: string;
            textRange: {
                startLine: number;
            };
        };
    }[];
}

