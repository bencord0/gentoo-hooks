# Gentoo Hooks

This is a hack around portage's [`bashrc`][1] feature, that lets us
inject additional steps into the package lifecycle.

In our case, we add some rust code to the whole thing.

[1]: https://wiki.gentoo.org/wiki//etc/portage/bashrc
