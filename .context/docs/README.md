# 📚 External Documentation (Vendorizing)

This folder contains documentation from external libraries and APIs so AI agents can consult it locally without depending on internet access.

## 🎯 Purpose

Many LLMs have outdated or limited knowledge about specific libraries. By placing documentation here, the agent can:

1. Query up-to-date APIs
2. View recent code examples
3. Avoid suggesting deprecated code

## 📥 What to place here?

- **`llms.txt`**: Special documentation files for LLMs (e.g., `mui-llms.txt`, `vercel-llms.txt`)
- **API References**: Documentation exports from APIs you use
- **Changelogs**: Important change notes from critical dependencies
- **Migration guides**: If you're migrating from one version to another

## 💡 Examples of useful files

```
docs/
├── mui-llms.txt           # Material UI docs for LLMs
├── nextjs-app-router.md   # Next.js App Router guide
├── prisma-schema-guide.md # Prisma schema reference
├── tailwind-v4-changes.md # Tailwind v3 to v4 changes
└── api-internal.md        # Your internal API documentation
```

## 🔍 How to get llms.txt

Many modern libraries publish `.txt` files optimized for LLMs:

- **Vercel**: `https://vercel.com/docs/llms.txt`
- **Material UI**: Search in their repository
- **Others**: Check `/llms.txt` or `/docs/llms.txt` on the official site

## ⚠️ Notes

- Keep only the essentials to avoid overloading the context
- Update periodically when you update dependencies
- Prioritize the libraries you use most in the project
