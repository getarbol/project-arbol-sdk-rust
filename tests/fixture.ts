export function fixture() {
  const input = {
    type: 'object',
    additionalProperties: false,
    required: ['target'],
    properties: {
      target: {
        oneOf: [
          {
            type: 'object',
            additionalProperties: false,
            required: ['kind', 'id'],
            properties: { kind: { const: 'clinic' }, id: { type: 'string' } },
          },
          {
            type: 'object',
            additionalProperties: false,
            required: ['kind', 'id'],
            properties: { kind: { const: 'person' }, id: { type: ['string', 'null'] } },
          },
        ],
      },
    },
  };
  const operation = {
    operationId: 'agenda.appointments.create',
    summary: 'Local verification operation',
    description: 'Synthetic local verification only; never published.',
    'x-arbol-api': { namespace: 'agenda', resource: 'appointments', verb: 'create', id: null, query: null },
    'x-arbol-service-id': 'agenda',
    'x-arbol-publication': 'selected',
    'x-arbol-publish': { rest: true, mcp: true },
    'x-arbol-input-schema': input,
    'x-arbol-actor-kinds': ['user', 'service'],
    'x-effect': 'write',
    'x-required-scopes': ['scheduling:appointments:read', 'scheduling:appointments:write'],
    'x-scope-match': 'any',
    requestBody: { required: true, content: { 'application/json': { schema: input } } },
    responses: {
      '200': {
        description: 'Full owner envelope',
        content: {
          'application/json': {
            schema: {
              oneOf: [
                {
                  type: 'object',
                  required: ['success', 'data', 'error', 'metadata'],
                  properties: {
                    success: { const: true },
                    data: {
                      type: 'array',
                      items: {
                        type: 'object',
                        required: ['id', 'label'],
                        properties: { id: { type: 'string' }, label: { type: ['string', 'null'] } },
                      },
                    },
                    error: { type: 'null' },
                    metadata: {
                      type: 'object',
                      required: ['requestId'],
                      properties: { requestId: { type: 'string' } },
                    },
                  },
                },
                {
                  type: 'object',
                  required: ['success', 'data', 'error', 'metadata'],
                  properties: {
                    success: { const: false },
                    data: { type: 'null' },
                    error: {
                      type: 'object',
                      required: ['code', 'message'],
                      properties: { code: { const: 'DENIED' }, message: { type: 'string' } },
                    },
                    metadata: {
                      type: 'object',
                      required: ['requestId'],
                      properties: { requestId: { type: 'string' } },
                    },
                  },
                },
              ],
            },
          },
        },
      },
    },
  };
  // A compiled contract always lists the person flow's permissions; the CLI requests them at login.
  const arbolHuman = {
    type: 'oauth2',
    flows: {
      authorizationCode: {
        authorizationUrl: 'https://auth.example.invalid/authorize',
        tokenUrl: 'https://auth.example.invalid/oauth/token',
        scopes: {
          'scheduling:appointments:read': 'Read appointments',
          'scheduling:appointments:write': 'Write appointments',
        },
      },
    },
  };
  return {
    openapi: '3.1.0',
    info: { title: 'Verification only', version: '0.0.0' },
    components: { schemas: {}, securitySchemes: { arbolHuman } },
    paths: { '/v1/agenda/appointments': { post: operation } },
  };
}
